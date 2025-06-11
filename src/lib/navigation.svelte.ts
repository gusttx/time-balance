import { FileQuestion, LayoutGrid, NotebookText, Tag, type IconProps } from "@lucide/svelte";
import type { Component } from "svelte";

export enum Page {
    ACTIVITIES,
    ENTRIES,
    HOME,
    NOT_FOUND
}

export type PageInfo = {
    page: Page;
    name: string;
    icon: Component<IconProps>;
}

const NOT_FOUND_PAGE: PageInfo = {
    page: Page.NOT_FOUND,
    name: "Unknown",
    icon: FileQuestion
};

class PageMap {
    private map: Map<Page, PageInfo> = new Map();

    add(page: Page, name: string, icon: Component<IconProps>) {
        this.map.set(page, { page, name, icon });
        return this;
    }
    
    get(page: Page): Readonly<PageInfo> {
        return this.map.get(page) ?? NOT_FOUND_PAGE;
    }

    all(): Readonly<PageInfo[]> {
        return Array.from(this.map.values());
    }
}

class NavigationState {
    private page: Page = $state(Page.NOT_FOUND);

    get currentPage(): Page {
        return this.page;
    }

    navigate(page: Page) {
        this.page = page;
    }
}

export const navigation = new NavigationState();

export const pages = new PageMap()
    .add(Page.HOME, "Home", LayoutGrid)
    .add(Page.ACTIVITIES, "Activities", Tag)
    .add(Page.ENTRIES, "Entries", NotebookText);