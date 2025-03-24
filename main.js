import init, {
    save_image,
    fetch_images,
    initialize,
    send_nostr_msg,
    fetch_nostr_events,
    fetch_and_decrypt_local_messages,
    save_encrypted_msg,
    fetchmsg,
} from '../pkg/index.js';

let npub;
async function run() {
    await init();
    npub = await initialize();
    document.getElementById("npub").textContent = npub;
    // await refresh_local_messages();
}

await run();


async function refresh_local_messages() {
    let local_messages = await fetch_and_decrypt_local_messages();
    let container = document.getElementById("local_messages");
    container.innerHTML = "";
    local_messages.forEach(str => {
        let div = document.createElement("div");
        div.textContent = str;
        container.appendChild(div);
    });
}

async function refresh_remote_messages() {
    let events = await fetch_nostr_events(npub);
    console.log(events);
    let container = document.getElementById("remote_events");
    container.innerHTML = "";
    events.forEach(event => {
        let div = document.createElement("div");
        div.textContent = event.content + " at " + event.ts + " (" + event.id + ")";
        container.appendChild(div);
    });
}

document.getElementById("sb").addEventListener("click", async () => {
    let input = document.getElementById("inp").value;
    await send_nostr_msg(input);
    await refresh_local_messages();
});

document.getElementById("fetch").addEventListener("click", async () => {
    await refresh_remote_messages();
});

document.getElementById("readwrite").addEventListener("click", async () => {
    let task_1 = readWrite("first");
    let task_2 = readWrite("second");
    let task_3 = readWrite("third");
    let task_4 = readWrite("fourth");
    await Promise.all([task_1, task_2, task_3, task_4]);
});

async function readWrite(t) {
    for (let i = 0; i <= 1000; i++) {
        console.log(t, i);
        await fetchmsg();
        // await save_encrypted_msg("this is an encrypted msg");
    }
}

document.getElementById("upload").addEventListener("click", async () => {
    const file = document.getElementById("file_input").files[0];
    if (!file) return;

    const name = file.name;
    const bytes = await file.arrayBuffer();
    const data = new Uint8Array(bytes);

    await save_image(name, data);
    console.log("upload successful");
});

document.getElementById("fetch_images").addEventListener("click", async () => {
    let files = await fetch_images();
    let container = document.getElementById("files");
    container.innerHTML = "";
    files.forEach(file => {
        let arr = new Uint8Array(file.bytes);
        let blob = new Blob([arr]);
        let url = URL.createObjectURL(blob);

        console.log("file", file, url, blob);
        let img = document.createElement("img");
        img.src = url;
        container.appendChild(img);
    });
});
