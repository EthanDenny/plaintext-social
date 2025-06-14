function handleLike(e) {
  const parts = e.target.innerHTML.split(" ");

  if (parts[0] === "🩶") {
    e.target.innerHTML = `❤️ ${parseInt(parts[1]) + 1}`;
  } else if (parts[0] === "❤️") {
    e.target.innerHTML = `🩶 ${parseInt(parts[1]) - 1}`;
  }
}

function handleShare(id) {
  const url = `${window.location.origin}/message/${id}`;
  navigator.clipboard
    .writeText(url)
    .then(() => {
      alert("Link copied to clipboard");
    })
    .catch((err) => {
      console.error("Failed to copy: ", err);
    });
}
