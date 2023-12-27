/// Only works in async functions, simply waits some time
function sleep(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }


async function depotEntryTableDeleteEntry() {
    let hash = this.event.target.dataset.hash;
    let sucessful = await invoke("depot_entry_delete", { depotEntryHash: hash });
    // TODO ^ use return value
    location.reload();	 // reload the page, so the deletion is rendered to UI
}

function depotEntryTableGetHtml() { depotEntryTableReloadHtml(this.event.target.dataset.hash); }

async function depotEntryTableReloadHtml(hash) {
    var html = await invoke("depot_entry_get_table_html", { depotEntryHash: hash });
    document.getElementById("content").innerHTML = html;

    // scroll to this years table (bottom of page)
    // without the timeout, this would ignore the padding of content and not scroll far enough ..
    setTimeout(() => { window.scrollBy(0, document.getElementById("content").scrollHeight); }, 50);
}

async function depotEntryTableSetCell() {
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
    invoke("depot_entry_set_cell_value", {
        depotEntryHash: hash,
        field: field,
        value: this.event.target.textContent,
        year: parseInt(year),
        month: parseInt(month)
    });
}

/// add new year + reload table html
async function depotEntryTableAddYear() {
    var buttonElement = this.event.target;

    var hash = buttonElement.dataset.hash;
    var sucessful = await invoke("depot_entry_add_previous_year", { depotEntryHash: hash });
    console.log("depotEntryTableAddYear " + sucessful);

    if (!sucessful) {
        console.warn("Previous Year could not be added to this depotEntry: " + hash);
        var innerTextBefore = buttonElement.innerHTML;
        buttonElement.innerHTML = "An Error occurred";
        buttonElement.classList.add('error');
        await sleep(3000);
        buttonElement.innerHTML = innerTextBefore;	// Reset text
        buttonElement.classList.remove('error');
        return;
    }

    depotEntryTableReloadHtml(hash);
}

function depotEntryTableScrollToRow(rowId) {
    let elem = document.getElementById(rowId);
    elem.scrollIntoView({
        behavior: 'smooth',
        block: 'center',
    });
}
