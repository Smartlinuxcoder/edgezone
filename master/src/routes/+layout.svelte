<script>
    import "../app.css";
    let { children } = $props();
</script>

<div
    class="min-h-screen bg-gradient-to-b from-[#1e1e2e] via-[#1e1e2e] to-[#181825] overflow-hidden"
>
    <!-- Enhanced background elements -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
        <!-- Large gradient blobs -->
        <div
            class="absolute -top-[40%] -right-[20%] w-[80%] h-[80%] rounded-full
                    bg-gradient-to-br from-[#cba6f7]/10 via-[#89b4fa]/5 to-transparent blur-3xl"
        ></div>
        <div
            class="absolute -bottom-[20%] -left-[40%] w-[80%] h-[80%] rounded-full
                    bg-gradient-to-tr from-[#f5c2e7]/5 via-[#89b4fa]/5 to-transparent blur-3xl"
        ></div>

        <!-- Animated accent lines -->
        {#each Array(8) as _, i}
            <div
                class="absolute animate-float"
                style="top: {Math.random() * 100}%; left: {Math.random() *
                    100}%; 
                        animation-delay: -{Math.random() * 5}s; 
                        animation-duration: {5 + Math.random() * 5}s"
            >
                <div
                    class="w-[1px] h-[20px] bg-gradient-to-b from-[#89b4fa]/50 to-transparent"
                ></div>
            </div>
        {/each}

        <!-- Enhanced grid pattern -->
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
        0%,
        100% {
            transform: translateY(0);
            opacity: 0.5;
        }
        50% {
            transform: translateY(-20px);
            opacity: 0.2;
        }
    }

    .animate-float {
        animation: float 5s ease-in-out infinite;
    }
</style>
