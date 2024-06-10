// colorutils.js
// Ensure this utility is in the right directory and correctly imported where needed

export function lightenColor(hex, percent) {
    // Check if 'hex' is a valid hex color string
    if (!/^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/.test(hex)) {
        console.error('Invalid hex color:', hex);
        return '#000000'; // Return black or some default color if the input is invalid
    }
    
    let num = parseInt(hex.slice(1), 16),
        amt = Math.round(2.55 * percent),
        R = (num >> 16) + amt,
        G = ((num >> 8) & 0x00FF) + amt,
        B = (num & 0x0000FF) + amt;

    R = (R < 255) ? R : 255;
    G = (G < 255) ? G : 255;
    B = (B < 255) ? B : 255;
    
    const RR = (R.toString(16).length === 1) ? '0' + R.toString(16) : R.toString(16);
    const GG = (G.toString(16).length === 1) ? '0' + G.toString(16) : G.toString(16);
    const BB = (B.toString(16).length === 1) ? '0' + B.toString(16) : B.toString(16);
    
    return '#' + RR + GG + BB;
}
