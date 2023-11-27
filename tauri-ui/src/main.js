const { invoke } = window.__TAURI__.tauri;

document.getElementById("de1").addEventListener("click", async () => {
	var html = await invoke("get_depot_entry_table_html");
	document.getElementById("content").innerHTML = html;
});

async function getDepotEntryHtml() {
	var html = await invoke("get_depot_entry_table_html");
	document.getElementById("content").innerHTML = html;
}

async function onInvestingCellInput() {
	console.log(this.event.target.value);
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
