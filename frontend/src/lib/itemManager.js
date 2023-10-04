import { get } from "svelte/store";
import { push } from "svelte-spa-router";
import { working, state, apierror } from "./store";

export default class ItemManager {
  constructor(url, sortcol, sortdesc, limit, offset) {
    this.url = url;
    this.sortCol = sortcol;
    this.sortDesc = sortdesc;
    this.limit = limit;
    this.offset = offset;
    this.searchText = "";
    this.result = [];
    this.error = "";
  }

  async search() {
    try {
      working.set(true);
      apierror.set("");
      this.error = "";
      let query = `q=${this.searchText}&sortcol=${this.sortCol}&desc=${this.sortDesc}&limit=${this.limit}&offset=${this.offset}`;

      const r = await fetch(`${this.url}?${query}`, {
        headers: {
          Authorization: "Bearer " + get(state).token,
        },
      });
      const j = await r.json();

      if (r.status !== 200) {
        this.error = j.detail;
        let token = get(state).token;
        if (!token || this.error === "Invalid token") {
          state.set({ username: "", token: "" });
          apierror.set("");
          this.error = "";
          localStorage.removeItem("state");
          push("/login");
        }

        apierror.set(this.error);
        console.log("url:", this.url);
        console.log(
          `searching text: ${this.searchText} sortcol: ${this.sortCol} desc: ${this.sortDesc}`
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
