const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
	// greetInputEl = document.querySelector("#greet-input");
	// greetMsgEl = document.querySelector("#greet-msg");
	// document.querySelector("#greet-form").addEventListener("submit", (e) => {
	//   e.preventDefault();
	//   greet();
	// });
});

document.getElementById("nav_acc_table").addEventListener("click", async () => {
	var html = await invoke("get_accounting_table_html");
	// console.log(html);
	document.getElementById("content").innerHTML = html;

	// var head = document.getElementsByTagName('head')[0];
	// var script = document.createElement('script');
	// script.src = 'main.js';
	// head.appendChild(script);
});

async function on_accounting_cell_blur() {
	// console.log(this);

	var html = await invoke("get_accounting_table_html");
	document.getElementById("content").innerHTML = html;
}
