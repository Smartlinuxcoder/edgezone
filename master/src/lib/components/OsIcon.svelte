<script>
    import LinuxLogo from 'phosphor-svelte/lib/LinuxLogo';
    import WindowsLogo from 'phosphor-svelte/lib/WindowsLogo';
    import AppleLogo from 'phosphor-svelte/lib/AppleLogo';
    import Question from 'phosphor-svelte/lib/Question';
    import Diamond from 'phosphor-svelte/lib/Diamond'; // For Arch
    import Spiral from 'phosphor-svelte/lib/Spiral'; // For Debian
    import Circle from 'phosphor-svelte/lib/Circle'; // For Ubuntu
    import Horse from 'phosphor-svelte/lib/Horse'; // For Fedora
    import Cloud from 'phosphor-svelte/lib/Cloud'; // For CentOS
    import Mountains from 'phosphor-svelte/lib/Mountains'; // For Alpine
    import Bug from 'phosphor-svelte/lib/Bug'; // For Gentoo
    import Leaf from 'phosphor-svelte/lib/Leaf'; // For OpenSUSE
    import Drop from 'phosphor-svelte/lib/Drop'; // For Void Linux
    import Package from 'phosphor-svelte/lib/Package'; // For NixOS

    export let os = '';
    export let distro = '';
    export let size = 24; // Default to larger size
    
    function getIcon(osString, distroString = '') {
        if (!osString) return Question;
        const lower = osString.toLowerCase();
        const distroLower = distroString.toLowerCase();

        const icons = {
            // Arch and friends
            'arch': { component: Diamond, color: '#89b4fa', weight: 'duotone' },
            'manjaro': { component: Diamond, color: '#a6e3a1', weight: 'duotone' },
            'endeavour': { component: Diamond, color: '#fab387', weight: 'duotone' },
            
            // Debian and friends
            'debian': { component: Spiral, color: '#f38ba8', weight: 'duotone' },
            'ubuntu': { component: Circle, color: '#fab387', weight: 'duotone' },
            'mint': { component: Spiral, color: '#a6e3a1', weight: 'duotone' },
            'pop!_os': { component: Circle, color: '#cba6f7', weight: 'duotone' },
            
            // Red Hat and friends
            'fedora': { component: Horse, color: '#89b4fa', weight: 'duotone' },
            'centos': { component: Cloud, color: '#cba6f7', weight: 'duotone' },
            'rhel': { component: Horse, color: '#f38ba8', weight: 'duotone' },
            
            // Other Linux
            'alpine': { component: Mountains, color: '#94e2d5', weight: 'duotone' },
            'gentoo': { component: Bug, color: '#89b4fa', weight: 'duotone' },
            'opensuse': { component: Leaf, color: '#a6e3a1', weight: 'duotone' },
            'void': { component: Drop, color: '#89dceb', weight: 'duotone' },
            'nixos': { component: Package, color: '#89b4fa', weight: 'duotone' },
            
            // Generics
            'linux': { component: LinuxLogo, color: '#a6e3a1', weight: 'duotone' },
            'windows': { component: WindowsLogo, color: '#89dceb', weight: 'duotone' },
            'macos': { component: AppleLogo, color: '#f5c2e7', weight: 'duotone' },
            'darwin': { component: AppleLogo, color: '#f5c2e7', weight: 'duotone' },
            'default': { component: Question, color: '#6c7086', weight: 'regular' }
        };

        for (const [key, value] of Object.entries(icons)) {
            if (distroLower.includes(key)) return value;
        }

        for (const [key, value] of Object.entries(icons)) {
            if (lower.includes(key)) return value;
        }
        
        return icons.default;
    }

    $: iconData = getIcon(os, distro);
</script>

<svelte:component 
    this={iconData.component} 
    size={size} 
    color={iconData.color} 
    weight={iconData.weight} 
/>
