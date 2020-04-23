import { LitElement, html } from 'lit-element';

export class DemoApp extends LitElement {
  static get properties() {
    return {
      selectedOfferId: {
        type: String,
      },
    };
  }

  render() {
    return html`
      <hcmc-create-offer></hcmc-create-offer>
      <hcmc-pending-offer-list
        @offer-selected=${(e) =>
          (this.selectedOfferId = e.detail.transactionId)}
      ></hcmc-pending-offer-list>
      ${this.selectedOfferId
        ? html`<hcmc-offer-detail
            .transactionId=${this.selectedOfferId}
          ></hcmc-offer-detail>`
        : html``}
    `;
  }
}
