function $(id) {
  return document.getElementById(id);
}

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

const maxLength = 160;

function updateCharCount(target) {
  const charCount = $("char-count");
  const messageInput = $("post-button");

  const currentLength = target.value.length;

  charCount.textContent = `${currentLength}/${maxLength}`;

  if (currentLength > maxLength) {
    charCount.style.color = "red";
    messageInput.disabled = true;
  } else {
    charCount.style.color = "";
    messageInput.disabled = false;
  }
}

document.addEventListener("DOMContentLoaded", function () {
  var input = $("message-input");
  if (input) updateCharCount(input);
});
