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
      user_name: localStorage.getItem("user_name"),
      content: $("post-input").value,
    }),
    headers: {
      "Content-Type": "application/json",
    },
  }).then((response) => {
    if (response.ok) {
      input.value = "";
      document.getElementById("char-count").textContent = "0/160";
      window.location.reload();
    }
  });
}

function createReply(string_id) {
  const id = parseInt(string_id, 10);
  const input = $("post-input");

  fetch("/reply", {
    method: "POST",
    body: JSON.stringify({
      user_name: localStorage.getItem("user_name"),
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
      window.location.reload();
    }
  });
}

function login() {
  const user_name = $("user_name").value;

  fetch("/login", {
    method: "POST",
    body: JSON.stringify({
      user_name,
    }),
    headers: {
      "Content-Type": "application/json",
    },
  }).then((response) => {
    if (response.ok) {
      const goto_feed = () => {
        localStorage.setItem("user_name", user_name);
        window.location.href = "/feed";
      };

      response.json().then((data) => {
        console.log(data);

        if (data.new_account) {
          let newUser = prompt("New account, please enter a display name:");
          if (newUser && newUser.trim() !== "") {
            fetch("/user/new", {
              method: "POST",
              body: JSON.stringify({
                user_name,
                display_name: newUser.trim(),
              }),
              headers: {
                "Content-Type": "application/json",
              },
            }).then((response) => {
              if (response.ok) {
                goto_feed();
              } else {
                alert("Failed to create new account. Please try again.");
              }
            });
          }
        } else {
          goto_feed();
        }
      });
    } else {
      alert("Login failed. Please try again.");
    }
  });
}
