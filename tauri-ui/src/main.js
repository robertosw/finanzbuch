const { invoke } = window.__TAURI__.tauri;

document.getElementById("de1").addEventListener("click", async () => {
	var html = await invoke("get_investing_table_html");
	document.getElementById("content").innerHTML = html;
});

async function getDepotEntryHtml() {
	var html = await invoke("get_investing_table_html");
	document.getElementById("content").innerHTML = html;
}

async function onInvestingCellBlur() {
	// console.log(this);
	invoke("set_investing_month_field", { field: "Amount" });
}
