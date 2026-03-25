// /public/google_auth.js
window.googleCallback = ({ credential }) => {
    fetch("/auth/google", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ credential }),
    });
};  