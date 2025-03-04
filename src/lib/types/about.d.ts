import type { Icon } from "lucide-svelte";

export interface Developer {
    name: string;
    image: string;
    role: string;
    socials?: SocialLink[];
}

export interface SocialLink {
    name: string;
    icon: typeof Icon;
    link: string;
}