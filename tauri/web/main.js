const { invoke } = window.__TAURI__.tauri;

window.onload = async () => {
	var html = await invoke("get_depot_entry_list_html");
	document.getElementById("depotEntryList").innerHTML = html;
}

async function getDepotEntryTableHtml() {
	console.log(this.event);
	var html = await invoke("get_depot_entry_table_html", { depotEntryHash: this.event.srcElement.name });
	document.getElementById("content").innerHTML = html;
}

async function setDepotEntryTableCell() {
	var [field_type, year, month] = this.event.target.id.split('-');
	var field = "";

	switch (field_type) {
		case "itp":
			field = "PricePerUnit";
			break;

		case "its":
			field = "Amount";
			break;

		case "ita":
			field = "AdditionalTransactions";
			break;
	}

	invoke("set_depot_entry_table_cell", {
		depotEntryHash: this.event.target.name,
		field: field,
		value: this.event.target.value,
		year: parseInt(year),
		month: parseInt(month)
	});
}
