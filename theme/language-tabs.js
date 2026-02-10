let isGlobalLanguageTabSyncEnabled = false;
let languageTabsSyncToggleButtonElement = null;

function getLanguageTabButtons(languageTabsContainerElement) {
  return Array.from(
    languageTabsContainerElement.querySelectorAll(".language-tabs-trigger"),
  );
}

function getLanguageTabPanels(languageTabsContainerElement) {
  return Array.from(
    languageTabsContainerElement.querySelectorAll(".language-tabs-panel"),
  );
}

function activateTabValueInContainer(
  languageTabsContainerElement,
  selectedTabValue,
) {
  const tabButtons = getLanguageTabButtons(languageTabsContainerElement);
  const tabPanels = getLanguageTabPanels(languageTabsContainerElement);
  const hasSelectedTabValue = tabButtons.some(
    (tabButton) => tabButton.dataset.languageTabsValue === selectedTabValue,
  );
  if (!hasSelectedTabValue) {
    return false;
  }

  for (const tabButton of tabButtons) {
    const isActiveTab =
      tabButton.dataset.languageTabsValue === selectedTabValue;
    tabButton.classList.toggle("is-active", isActiveTab);
    tabButton.setAttribute("aria-selected", isActiveTab ? "true" : "false");
    tabButton.setAttribute("tabindex", isActiveTab ? "0" : "-1");
  }

  for (const tabPanel of tabPanels) {
    tabPanel.classList.toggle(
      "is-active",
      tabPanel.dataset.languageTabsValue === selectedTabValue,
    );
  }

  return true;
}

function getLanguageTabContainerElements() {
  return Array.from(document.querySelectorAll(".language-tabs"));
}

function activateTabValueGlobally(selectedTabValue) {
  for (const languageTabsContainerElement of getLanguageTabContainerElements()) {
    activateTabValueInContainer(languageTabsContainerElement, selectedTabValue);
  }
}

function activateTabValueForGroup(groupIdentifier, selectedTabValue) {
  const selector = `.language-tabs[data-language-tabs-group="${groupIdentifier}"]`;
  for (const languageTabsContainerElement of document.querySelectorAll(
    selector,
  )) {
    activateTabValueInContainer(languageTabsContainerElement, selectedTabValue);
  }
}

function getFirstActiveTabValue() {
  for (const languageTabsContainerElement of getLanguageTabContainerElements()) {
    for (const tabButton of getLanguageTabButtons(languageTabsContainerElement)) {
      if (tabButton.classList.contains("is-active")) {
        return tabButton.dataset.languageTabsValue ?? null;
      }
    }
  }

  return null;
}

function updateLanguageTabsSyncToggleButtonState() {
  if (!(languageTabsSyncToggleButtonElement instanceof HTMLButtonElement)) {
    return;
  }

  languageTabsSyncToggleButtonElement.classList.toggle(
    "is-active",
    isGlobalLanguageTabSyncEnabled,
  );
  languageTabsSyncToggleButtonElement.setAttribute(
    "aria-pressed",
    isGlobalLanguageTabSyncEnabled ? "true" : "false",
  );
  languageTabsSyncToggleButtonElement.textContent = isGlobalLanguageTabSyncEnabled
    ? "Sync Tabs: On"
    : "Sync Tabs: Off";
}

function createLanguageTabsSyncToggle() {
  const languageTabsContainerElements = getLanguageTabContainerElements();
  if (languageTabsContainerElements.length < 2) {
    return;
  }

  const headerButtonContainerElement =
    document.querySelector("#mdbook-menu-bar .right-buttons") ??
    document.querySelector("#mdbook-menu-bar .left-buttons");
  if (!(headerButtonContainerElement instanceof HTMLElement)) {
    return;
  }

  const syncToggleButtonElement = document.createElement("button");
  syncToggleButtonElement.type = "button";
  syncToggleButtonElement.className = "language-tabs-header-toggle";
  syncToggleButtonElement.title = "Sync selected language across all tab groups";
  syncToggleButtonElement.addEventListener("click", () => {
    isGlobalLanguageTabSyncEnabled = !isGlobalLanguageTabSyncEnabled;
    updateLanguageTabsSyncToggleButtonState();
    if (!isGlobalLanguageTabSyncEnabled) {
      return;
    }

    const firstActiveTabValue = getFirstActiveTabValue();
    if (!firstActiveTabValue) {
      return;
    }
    activateTabValueGlobally(firstActiveTabValue);
  });

  languageTabsSyncToggleButtonElement = syncToggleButtonElement;
  updateLanguageTabsSyncToggleButtonState();
  headerButtonContainerElement.prepend(syncToggleButtonElement);
}

function activateDefaultTab(languageTabsContainerElement) {
  const groupIdentifier =
    languageTabsContainerElement.dataset.languageTabsGroup;
  if (!groupIdentifier) {
    return;
  }

  const tabButtons = getLanguageTabButtons(languageTabsContainerElement);
  if (tabButtons.length === 0) {
    return;
  }

  const defaultTabButton =
    tabButtons.find((tabButton) => tabButton.classList.contains("is-active")) ??
    tabButtons[0];
  const defaultTabValue = defaultTabButton.dataset.languageTabsValue;
  if (defaultTabValue) {
    activateTabValueForGroup(groupIdentifier, defaultTabValue);
  }
}

function handleTabClick(event) {
  const tabButton = event.currentTarget;
  if (!(tabButton instanceof HTMLElement)) {
    return;
  }

  const languageTabsContainerElement = tabButton.closest(".language-tabs");
  const groupIdentifier =
    languageTabsContainerElement?.dataset.languageTabsGroup;
  const selectedTabValue = tabButton.dataset.languageTabsValue;

  if (!groupIdentifier || !selectedTabValue) {
    return;
  }

  if (isGlobalLanguageTabSyncEnabled) {
    activateTabValueGlobally(selectedTabValue);
    return;
  }

  activateTabValueForGroup(groupIdentifier, selectedTabValue);
}

function handleTabKeyboardNavigation(event) {
  if (!(event.currentTarget instanceof HTMLElement)) {
    return;
  }

  const tabButton = event.currentTarget;
  const languageTabsContainerElement = tabButton.closest(".language-tabs");
  if (!languageTabsContainerElement) {
    return;
  }

  const tabButtons = getLanguageTabButtons(languageTabsContainerElement);
  const currentTabButtonIndex = tabButtons.indexOf(tabButton);
  if (currentTabButtonIndex === -1) {
    return;
  }

  let nextTabButtonIndex = -1;
  if (event.key === "ArrowRight") {
    nextTabButtonIndex = (currentTabButtonIndex + 1) % tabButtons.length;
  } else if (event.key === "ArrowLeft") {
    nextTabButtonIndex =
      (currentTabButtonIndex - 1 + tabButtons.length) % tabButtons.length;
  }

  if (nextTabButtonIndex === -1) {
    return;
  }

  event.preventDefault();
  tabButtons[nextTabButtonIndex]?.focus();
  tabButtons[nextTabButtonIndex]?.click();
}

function initializeLanguageTabs() {
  const languageTabsContainerElements = getLanguageTabContainerElements();

  for (const languageTabsContainerElement of languageTabsContainerElements) {
    activateDefaultTab(languageTabsContainerElement);
    for (const tabButton of getLanguageTabButtons(
      languageTabsContainerElement,
    )) {
      tabButton.addEventListener("click", handleTabClick);
      tabButton.addEventListener("keydown", handleTabKeyboardNavigation);
    }
  }

  createLanguageTabsSyncToggle();
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initializeLanguageTabs);
} else {
  initializeLanguageTabs();
}
