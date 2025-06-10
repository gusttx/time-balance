export enum Page {
    ACTIVITIES,
    HOME,
    NOT_FOUND
}

class NavigationState {
    #page: Page = $state(Page.NOT_FOUND);

    get currentPage(): Page {
        return this.#page;
    }

    navigate(page: Page) {
        this.#page = page;
    }
}

export const navigation = new NavigationState();