const { invoke } = window.__TAURI__.tauri;

window.onload = async () => {
	var html = await invoke("get_depot_entry_list_html");
	document.getElementById("depotEntryList").innerHTML = html;
}

async function getDepotEntryHtml() {
	console.log(this.event);
	var html = await invoke("get_depot_entry_table_html", { depotEntryName: this.event.srcElement.innerHTML });
	document.getElementById("content").innerHTML = html;
}

async function onInvestingCellInput() {
	var [field_type, year, month] = this.event.target.id.split('-');

	switch (field_type) {
		case "itp":
			invoke("set_depot_entry_table_cell", { field: "PricePerUnit", value: this.event.target.value, year: parseInt(year), month: parseInt(month) });
			break;

		case "its":
			invoke("set_depot_entry_table_cell", { field: "Amount", value: this.event.target.value, year: parseInt(year), month: parseInt(month) });
			break;

		case "ita":
			invoke("set_depot_entry_table_cell", { field: "AdditionalTransactions", value: this.event.target.value, year: parseInt(year), month: parseInt(month) });
			break;
	}

}
