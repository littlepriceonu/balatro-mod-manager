<script lang="ts">
	import Profile from './Profile.svelte';
	import { BookOpen, Coffee, Github, Globe} from "lucide-svelte";
	import { open } from "@tauri-apps/plugin-shell";
	import { Confetti } from "svelte-confetti";
	import type { Developer } from "$lib/types/about";

	let showConfetti = false;

	let buttonRect: DOMRect;

	const handleKofiClick = async (event: MouseEvent) => {
		const button = event.currentTarget as HTMLButtonElement;
		buttonRect = button.getBoundingClientRect();
		showConfetti = true;
		setTimeout(() => (showConfetti = false), 2000);
		try {
			await open("https://ko-fi.com/skyline69/goal?g=0");
		} catch (error) {
			console.error("Failed to open URL:", error);
		}
	};

	const developers: Developer[] = [
		{
			name: "Efe / Skyline",
			image: "/images/profile-pictures/skyline.jpg",
			role: "The Creator and Founder of BMM",
			socials: [
				{
					name: "GitHub",
					icon: Github,
					link: "https://github.com/skyline69",
				},
				{
					name: "Website",
					icon: Globe,
					link: "https://dasguney.com/",
				}
			]
		},
		{
			name: "Little",
			image: "/images/profile-pictures/little.gif",
			role: "Developer & UI Designer",
			socials: [
				{
					name: "GitHub",
					icon: Github,
					link: "https://github.com/littlepriceonu",
				},
				{
					name: "Website",
					icon: Globe,
					link: "https://littlepriceonu.com/",
				}
			]
		},
	];

</script>

<div class="container default-scrollbar">
	<div class="about-container">
		<div class="content">
			<h2>About Balatro Mod Manager</h2>
	
			<div>
				<h3>What is BMM?</h3>
				<p>
					Balatro Mod Manager (BMM) is a tool designed to help you manage
					and install mods for the game Balatro. It provides an
					easy-to-use interface for mod management while maintaining the
					game's unique aesthetic.
				</p>
			</div>
	
			<div>
				<h3>Features</h3>
				<ul>
					<li>Easy mod installation and management</li>
					<li>Automatic game path detection</li>
					<li>Mod compatibility checking</li>
					<li>Clean, pixel-perfect interface</li>
				</ul>
			</div>
	
			<div class="button-container">
				<button
					class="wiki-button"
					on:click={() =>
						open("https://balatromods.miraheze.org/wiki/Main_Page")}
				>
					<BookOpen size={20} />
					<span>Visit Wiki</span>
				</button>
				<button class="kofi-button" on:click={handleKofiClick}>
					<div class="confetti-container">
						{#if showConfetti}
							<Confetti
								x={[0, 1]}
								y={[0, 1]}
								duration={4000}
								amount={50}
							/>
						{/if}
					</div>
					<Coffee size={20} />
					<span>Support on Ko-fi</span>
				</button>
			</div>
	
			<p id="versiontext">Current version: v0.1.7</p>
		</div>
	
		<div class="profile-section default-scrollbar">
			<p >Developers</p>

			{#each developers as developer }
				<Profile {developer} />
			{/each}
		</div>
	</div>
</div>

<style>	

	.about-container {
		display: flex;
		justify-content: space-between;
		gap: 2rem;

		height: 100%;
	}

	.profile-section {
		width: 30%;
		height: 100%;
		padding-top: 1rem;

		border-left: #f7f1e4 solid 2px;

		box-sizing: border-box;
		display: flex;
		flex-direction: column;
		align-items: center;

		overflow: auto;

		padding-bottom: 12px;

		&::-webkit-scrollbar {
			width: 8px !important;
		}
	}

	:global(.profile-section > div) {
		margin-top: 1rem;
	}

	.profile-section > p {
		margin: 0;
		font-size: 1.5rem;
	}

	.content {
		width: 70%;
		padding: 0 2rem;
	}

	.wiki-button {
		background-color: #fdcf51;
		border: 4px solid #f7f1e4;
		border-radius: 8px;
		color: #000;
		padding: 0.5rem 1rem;
		font-family: "M6X11", sans-serif;
		font-size: 1.2rem;
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
		margin: 0;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		position: relative;
	}

	.wiki-button:hover {
		background-color: #fde700;
		transform: scale(1.05);
	}

	.wiki-button:active {
		transform: scale(0.95);
	}

	.kofi-button {
		background-color: #29abe0;
		border: 4px solid var(--white);
		border-radius: 8px;
		color: #fff;
		padding: 0.5rem 1rem;
		font-family: "M6X11", sans-serif;
		font-size: 1.2rem;
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
		margin: 0;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		position: relative;
	}

	.kofi-button:hover {
		background-color: #13a3e1;
		transform: scale(1.05);
	}

	.kofi-button:active {
		transform: scale(0.95);
	}

	h2 {
		font-size: 2.5rem;
		margin-bottom: 1rem;
		color: #fdcf51;
	}

	h3 {
		font-size: 1.8rem;
		margin-bottom: 0.5rem;
		color: #fdcf51;
	}

	p {
		font-size: 1.2rem;
		margin-bottom: 1rem;
	}

	#versiontext {
		color: #fde700;
		margin-top: 1rem;
	}

	ul {
		list-style-type: disc;
		margin-left: 1rem;
		margin-bottom: 1rem;
	}

	li {
		font-size: 1.2rem;
		margin-bottom: 0.5rem;
	}
	.button-container {
		display: flex;
		gap: 1rem;
		margin: 1rem 0;
	}

	.confetti-container {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		pointer-events: none;
	}

	@media (max-width: 1160px) {
		p {
			font-size: 1rem;
		}
		li {
			font-size: 1rem;
		}

		h2 {
			font-size: 2rem;
		}
		h3 {
			font-size: 1.5rem;
		}
		.wiki-button {
			font-size: 1rem;
			padding: 0.4rem 0.8rem;
		}
		.kofi-button {
			font-size: 1rem;
			padding: 0.4rem 0.8rem;
		}
	}
</style>
