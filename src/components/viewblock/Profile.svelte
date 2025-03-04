<script lang="ts">
    import type { Developer, SocialLink } from "$lib/types/about";
	import { open } from "@tauri-apps/plugin-shell";

    interface Props {
        developer: Developer
    }

    let { developer }: Props = $props();

    let defaultSocial = $derived.by(()=>{
        if (!developer.socials) return;

        let github = developer.socials.find(social => social.name === "GitHub");

        if (github) return github;

        return developer.socials[0];
    })

    function openSocial(social?: SocialLink) {
        if (!social) return

        open(social.link);
    }

</script>

<div class="developer-profile">
    <button onclick={()=>{openSocial(defaultSocial)}} class="pfp-container" class:has-social={!!defaultSocial}>
        <img class="profile-picture" src="{developer.image}" alt="{developer.name}'s Profile Picture.">
    </button>

    <p class="developer-name">
        {developer.name}
    </p>
    <p class="developer-role">
        {developer.role}
    </p>

    {#if developer.socials}
        <hr class="divider"/>

        <div class="socials">
                {#each developer.socials as social}
                    {@const Icon = social.icon}

                    <button onclick={()=>{openSocial(social)}} class="social-container" class:has-social={!!social}>
                        <Icon color="#f7f1e4" name="{developer.name}'s {social.name}" />
                    </button>
                {/each}
        </div>

        <hr class="divider"/>
    {/if}

</div>

<style>
    * {
        margin: 0;
    }

    .divider {
        width: 70%;
        border: 1px #e2ded4 solid;
    }

    .social-container {
        width: 25px;
        height: 25px;
        padding: 0;

        background: transparent;
        border: none;

        display: flex;
        justify-content: center;
        align-items: center;

        cursor: pointer;

        transition: 0.3s all ease-in-out;
    }

    .social-container:hover {
        scale: 1.05;
        translate: 0 -2px;
    }

    .social-container:active {
        scale: 0.95;
        translate: 0 1px;
    }

    .socials {
        display: flex;
        gap: 1rem;
        justify-content: center;
        align-items: center;

        width: 70%;
        height: 35px
    }

    .developer-role {
        font-size: 0.9rem;
        color: #f7f1e4;
        width: 80%;
        text-align: center;
        margin-top: 0.2rem;
        margin-bottom: 0.2rem;
    }

    .developer-name {
        font-size: 1.5rem;
        line-height: 1.3rem;
        color: #f7f1e4;
        text-align: center;

        margin-top: 0.5rem;
    }

    .developer-profile {
        width: 90%;
        height: 300px;

        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .pfp-container {
        width: 175px;
        height: 175px;

        border-radius: 9999px;
        border: 4px solid #f7f1e4;

        overflow: hidden;

        padding: 0;
    }

    .pfp-container.has-social {
        cursor: pointer;

        transition: ease-in-out 300ms all;
    }

    .pfp-container.has-social:active {
        scale: 0.95;
    }

    .pfp-container.has-social:hover {
        translate: 0 -3px;
    }

    .profile-picture {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }
</style>