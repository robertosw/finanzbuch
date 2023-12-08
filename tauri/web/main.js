const { invoke } = window.__TAURI__.tauri;

window.onload = async () => {
	var html = await invoke("get_depot_entry_list_html");
	document.getElementById("depotEntryList").innerHTML = html;
}

async function getDepotEntryTableHtml() { replaceDepotEntryTableHtml(this.event.srcElement.name);}

async function replaceDepotEntryTableHtml(hash) {
	var html = await invoke("get_depot_entry_table_html", { depotEntryHash: hash });
	document.getElementById("content").innerHTML = html;

	// scroll to this years table (bottom of page)
	// without the timeout, this would ignore the padding of content and not scroll far enough ..
	setTimeout(() => { window.scrollBy(0, document.getElementById("content").scrollHeight); }, 50);
}

async function setDepotEntryTableCell() {
	var [field_type, year, month, hash] = this.event.target.id.split('-');
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

	// TODO check for return value
	invoke("set_depot_entry_table_cell", {
		depotEntryHash: hash,
		field: field,
		value: this.event.target.textContent,
		year: parseInt(year),
		month: parseInt(month)
	});
}

async function addDepotTable() {
	// add new year + reload table html
	var hash = this.event.srcElement.name;
	var sucessful = await invoke("add_depot_entrys_previous_year", { depotEntryHash: hash });
	console.log("addDepotTable " + sucessful);

	if (!sucessful) {
		console.error("Previous Year could not be added to this depotEntry: " + this.event.target.name);
		// TODO what to do here?
		return;
	}

	replaceDepotEntryTableHtml(hash);
	// TODO somehow, data that is written in the new table, is saved in 2023 table
}

function scrollDepotTableToRow(rowId) {
	let elem = document.getElementById(rowId);
	elem.scrollIntoView({
		behavior: 'smooth',
		block: 'center',
	});
}
