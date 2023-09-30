import { get } from "svelte/store";
import { working, state, apierror } from "./store";

export default class ItemManager {
  constructor(url) {
    this.url = url;
    this.sortCol = "id";
    this.sortDesc = false;
    this.searchText = "";
    this.result = [];
    this.error = "";
  }

  async search() {
    try {
      working.set(true);
      apierror.set("");
      this.error = "";
      let query = `q=${this.searchText}&sortcol=${this.sortCol}&desc=${this.sortDesc}`;

      const r = await fetch(`${this.url}?${query}`, {
        headers: {
          Authorization: "Bearer " + get(state).token,
        },
      });
      const j = await r.json();

      if (r.status !== 200) {
        this.error = j.detail;
        apierror.set(this.error);
        if (this.error === "Invalid token") {
          state.set({});
        }
        console.log("url:", this.url);
        console.log(
          `searching: ${this.searchText} sortcol: ${this.sortCol} desc: ${this.sortDesc}`
        );
        console.log("error:", this.error);
      } else {
        this.result = j;
      }
    } catch (err) {
      console.log(err);
      console.log(
        `searching: ${this.searchText} sortcol: ${this.sortCol} desc: ${this.sortDesc}`
      );
      console.log("url:", this.url);
      this.error = "API: Error in fetching data.";
      apierror.set(this.error);
    } finally {
      working.set(false);
    }
  }

  async create(item) {
    try {
      working.set(true);
      this.error = "";
      //console.log($state.token);
      const r = await fetch(this.url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer " + get(state).token,
        },
        body: JSON.stringify(item),
      });
      const js = await r.json();
      //console.log(js);

      if (r.status !== 200) {
        this.error = js.detail;
      } else {
        await this.search();
      }
    } catch (err) {
      console.log(err);
    }
    working.set(false);
  }

  async modify(item) {
    try {
      working.set(true);
      this.error = "";
      console.log("modify:", this.url);
      const r = await fetch(this.url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer " + get(state).token,
        },
        body: JSON.stringify(item),
      });
      const js = await r.json();
      console.log(js);

      if (r.status !== 200) {
        this.error = js.detail;
      } else {
        await this.search();
      }
    } catch (err) {
      console.log(err);
    }
    working.set(false);
  }

  async remove(id) {
    try {
      working.set(true);
      this.error = "";
      const r = await fetch(`${this.url}${id}`, {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer " + get(state).token,
        },
      });
      const js = await r.json();
      //console.log(js);

      if (r.status !== 200) {
        this.error = js.detail;
      } else {
        await this.search();
      }
    } catch (err) {
      console.log(err);
    }
    working.set(false);
  }
}
