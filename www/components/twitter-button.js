/**
 * Inspired by https://github.com/topheman/npm-registry-browser/blob/master/src/components/TwitterButton.js
 */

const defaultAttributes = {
  size: "l",
  lang: "en",
  dnt: false,
  buttonTitle: "Twitter Tweet Button",
  text: null,
  url: null,
  hashtags: null,
  via: null,
  related: null,
  className: null,
  style: null,
};

class TwitterButton extends HTMLElement {
  constructor() {
    super();
    this.initDefaultValues();
    const template = document.createElement("template");
    template.innerHTML = `
<style>
  :host {
    display: inline-block;
  }
</style>
<iframe allowtransparency="true" frameborder="0" scrolling="no" width="82px" height="28px"></iframe>
    `;
    const shadow = this.attachShadow({ mode: "open" });
    shadow.appendChild(template.content.cloneNode(true));
    this.render();
  }

  initDefaultValues() {
    Object.entries(defaultAttributes).forEach(
      ([attributeName, defaultValue]) => {
        this[attributeName] = this[attributeName] || defaultValue;
      }
    );
  }

  static get observedAttributes() {
    return Object.keys(defaultAttributes);
  }

  attributeChangedCallback(attrName, oldVal, newVal) {
    if (oldVal !== newVal) {
      this.render();
    }
  }

  render() {
    const params = [
      `size=${this.size}`,
      "count=none",
      `dnt=${this.dnt}`,
      `lang=${this.lang}`,
      this.text != null && `text=${encodeURIComponent(this.text)}`,
      this.url != null && `url=${encodeURIComponent(this.url)}`,
      this.hashtags != null && `hashtags=${encodeURIComponent(this.hashtags)}`,
      this.via != null && `via=${encodeURIComponent(this.via)}`,
      this.related != null && `related=${encodeURIComponent(this.related)}`,
    ]
      .filter(Boolean)
      .join("&");
    const iframe = this.shadowRoot.querySelector("iframe");
    iframe.src = `https://platform.twitter.com/widgets/tweet_button.html?${params}`;
    iframe.title = this.buttonTitle;
  }
}

Object.keys(defaultAttributes).forEach((attributeName) => {
  Object.defineProperty(TwitterButton.prototype, attributeName, {
    get() {
      return this.getAttribute(attributeName);
    },
    set(value) {
      if (typeof value === "undefined" || value === null) {
        this.removeAttribute(attributeName);
      } else {
        this.setAttribute(attributeName, value);
      }
    },
  });
});

customElements.define("twitter-button", TwitterButton);
