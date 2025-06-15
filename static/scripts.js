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
  const url = `${window.location.origin}/post/${id}`;
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
  const postInput = $("post-button");

  const currentLength = target.value.length;

  charCount.textContent = `${currentLength}/${maxLength}`;

  if (currentLength > maxLength) {
    charCount.style.color = "red";
    postInput.disabled = true;
  } else {
    charCount.style.color = "";
    postInput.disabled = false;
  }
}

document.addEventListener("DOMContentLoaded", function () {
  const input = $("post-input");
  if (input) updateCharCount(input);
});

function createPost() {
  const input = $("post-input");

  fetch("/post", {
    method: "POST",
    body: JSON.stringify({
      content: $("post-input").value,
    }),
    headers: {
      "Content-Type": "application/json",
    },
  }).then((response) => {
    if (response.ok) {
      input.value = "";
      document.getElementById("char-count").textContent = "0/160";
    }
  });
}

function createReply(string_id) {
  const id = parseInt(string_id, 10);
  const input = $("post-input");

  fetch("/reply", {
    method: "POST",
    body: JSON.stringify({
      content: $("post-input").value,
      parent_id: id,
    }),
    headers: {
      "Content-Type": "application/json",
    },
  }).then((response) => {
    if (response.ok) {
      input.value = "";
      document.getElementById("char-count").textContent = "0/160";
    }
  });
}
