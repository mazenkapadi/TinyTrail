async function shorten(validity) {
    const longUrl = document.getElementById('longUrl').value;
    const response = await fetch('/api/shorten', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ long_url: longUrl, validity })
    });
    const data = await response.json();
    document.getElementById('result').innerText = data.short_url;
}

// function shorten(option) {
//     const longUrl = document.getElementById("longUrl").value;
//     if (!longUrl) {
//         alert("Please enter a URL.");
//         return;
//     }
//
//     // Simulate URL shortening
//     const options = {
//         1: "Shortened URL (3 accesses, 24 hours): ",
//         2: "Shortened URL (7 days): ",
//         3: "Shortened URL (Forever): ",
//     };
//
//     const shortUrl = `${options[option]}https://tiny.tr/${Math.random().toString(36).substr(2, 6)}`;
//     document.getElementById("result").innerText = shortUrl;
// }
