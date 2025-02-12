<script>
    import "../app.css";
    let { children } = $props();
</script>

<div
    class="min-h-screen bg-gradient-to-b from-[#1e1e2e] via-[#1e1e2e] to-[#181825] overflow-hidden"
>
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
        <div
            class="absolute -top-[40%] -right-[20%] w-[80%] h-[80%] rounded-full
                    bg-gradient-to-br from-[#cba6f7]/10 via-[#89b4fa]/5 to-transparent blur-3xl"
        ></div>
        <div
            class="absolute -bottom-[20%] -left-[40%] w-[80%] h-[80%] rounded-full
                    bg-gradient-to-tr from-[#f5c2e7]/5 via-[#89b4fa]/5 to-transparent blur-3xl"
        ></div>

        {#each Array(15) as _, i}
            <div
                class="absolute animate-float opacity-0"
                style="top: {Math.random() * 100}%; left: {Math.random() *
                    100}%; 
                       animation-delay: -{Math.random() * 8}s; 
                       animation-duration: {8 + Math.random() * 7}s"
            >
                <div class="accent-line"></div>
            </div>
        {/each}

        <div class="absolute inset-0"></div>
        <div
            class="absolute inset-0 bg-[linear-gradient(to_right,#6e6c7e1a_1px,transparent_1px),linear-gradient(to_bottom,#6e6c7e1a_1px,transparent_1px)]
                        bg-[size:4rem_4rem]"
        ></div>
        <div
            class="absolute inset-0 bg-[linear-gradient(to_right,#6e6c7e05_1px,transparent_1px),linear-gradient(to_bottom,#6e6c7e05_1px,transparent_1px)]
                        bg-[size:1rem_1rem]"
        ></div>
    </div>
    <main class="px-4 py-8 md:py-12 relative">
        {@render children()}
    </main>
    
</div>


<style>
    :global(:root) {
        --ctp-rosewater: #f5e0dc;
        --ctp-flamingo: #f2cdcd;
        --ctp-pink: #f5c2e7;
        --ctp-mauve: #cba6f7;
        --ctp-red: #f38ba8;
        --ctp-maroon: #eba0ac;
        --ctp-peach: #fab387;
        --ctp-yellow: #f9e2af;
        --ctp-green: #a6e3a1;
        --ctp-teal: #94e2d5;
        --ctp-blue: #89b4fa;
        --ctp-sapphire: #74c7ec;
        --ctp-lavender: #b4befe;
        --ctp-text: #cdd6f4;
        --ctp-subtext1: #bac2de;
        --ctp-subtext0: #a6adc8;
        --ctp-overlay2: #9399b2;
        --ctp-overlay1: #7f849c;
        --ctp-overlay0: #6c7086;
        --ctp-surface2: #585b70;
        --ctp-surface1: #45475a;
        --ctp-surface0: #313244;
        --ctp-base: #1e1e2e;
        --ctp-mantle: #181825;
        --ctp-crust: #11111b;
    }

    .bg-noise {
        background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 400 400' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E");
    }

    main {
        position: relative;
        z-index: 1;
    }

    @keyframes subtle-fade {
        0% {
            opacity: 0.03;
        }
        50% {
            opacity: 0.05;
        }
        100% {
            opacity: 0.03;
        }
    }

    :global(.bg-gradient-radial) {
        animation: subtle-fade 10s ease-in-out infinite;
    }

    @keyframes float {
        0% {
            transform: translateY(100px) rotate(5deg);
            opacity: 0;
        }
        20% {
            opacity: 0.7;
        }
        50% {
            transform: translateY(-100px) rotate(-5deg);
            opacity: 0.3;
        }
        80% {
            opacity: 0.7;
        }
        100% {
            transform: translateY(-300px) rotate(5deg);
            opacity: 0;
        }
    }

    @keyframes pulse {
        0%, 100% {
            transform: scaleY(1) translateZ(0);
        }
        50% {
            transform: scaleY(1.2) translateZ(0);
        }
    }

    .accent-line {
        width: 1px;
        height: 40px;
        background: linear-gradient(
            180deg, 
            rgba(137, 180, 250, 0.7) 0%,
            rgba(203, 166, 247, 0.4) 50%,
            transparent 100%
        );
        box-shadow: 0 0 8px rgba(137, 180, 250, 0.3);
        transform-origin: center;
        animation: pulse 3s ease-in-out infinite;
    }

    .animate-float {
        will-change: transform, opacity;
        animation: float 12s ease-out infinite;
    }
</style>
