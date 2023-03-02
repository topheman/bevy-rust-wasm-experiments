if (typeof QRCode === "undefined") {
  throw new Error(
    "Missing `QRCode` function, please include `qrcode.min.js` as script tags before from https://unpkg.com/qrcodejs@1.0.0/qrcode.min.js"
  );
}

class QRCodeDisplay extends HTMLElement {
  constructor() {
    super();
    const shadow = this.attachShadow({ mode: "open" });
    const style = document.createElement("style");
    const run = document.createElement("div"); // wraps the qrcode that will be shown
    run.className = "run";
    const build = document.createElement("div"); // wraps the div where the qrcode is built
    build.className = "build";
    style.textContent = `
.build {
  display: none;
}
    `;
    shadow.appendChild(style);
    shadow.appendChild(run);
    shadow.appendChild(build);
    this.render();
  }

  static get observedAttributes() {
    return ["data", "width", "height", "wrap-anchor"];
  }

  attributeChangedCallback(attrName, oldVal, newVal) {
    if (oldVal !== newVal) {
      this.render();
    }
  }

  render() {
    const data = this.getAttribute("data");
    let wrapAnchor = false;
    try {
      wrapAnchor = JSON.parse(this.getAttribute("wrap-anchor"));
    } catch (e) {
      // eslint-disable-next-line no-console
      console.warn(
        "Wrong `wrap-anchor` attribute passed to `qrcode-display` (only accepts `true` or `false`)"
      );
      wrapAnchor = false;
    }
    if (data) {
      this.shadowRoot.querySelector(".build").innerHTML = "";
      /* eslint-disable */
      new QRCode(this.shadowRoot.querySelector(".build"), {
        text: this.getAttribute("data"),
        width: parseInt(this.getAttribute("width")) || 200,
        height: parseInt(this.getAttribute("height")) || 200,
        colorDark: "#900000",
      });
      /* eslint-enable */
      const img = this.shadowRoot.querySelector(".build img");
      // 😢
      setTimeout(() => {
        img.style.display = "initial";
      }, 0);
      img.title = data;
      this.shadowRoot.querySelector(".run").innerHTML = "";
      if (wrapAnchor) {
        const a = document.createElement("a");
        a.href = data;
        a.title = data;
        a.appendChild(img);
        this.shadowRoot.querySelector(".run").appendChild(a);
      } else {
        this.shadowRoot.querySelector(".run").appendChild(img);
      }
    }
  }
}

customElements.define("qrcode-display", QRCodeDisplay);
