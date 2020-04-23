import { LitElement, html, property } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { ApolloClientModule } from '@uprtcl/graphql';

import '@material/mwc-list';
import '@authentic/mwc-circular-progress';

import { GET_PENDING_OFFERS } from '../graphql/queries';
import { Transaction, Offer } from 'src/types';
import { sharedStyles } from './sharedStyles';

export class PendingOfferList extends moduleConnect(LitElement) {
  client!: ApolloClient<any>;

  @property({ type: Object, attribute: false })
  offers!: Offer[];

  static get styles() {
    return sharedStyles;
  }

  async firstUpdated() {
    this.client = this.request(ApolloClientModule.bindings.Client);

    const result = await this.client.query({
      query: GET_PENDING_OFFERS,
    });
    console.log(result);

    this.offers = result.data.myOffers;
  }

  offerSelected(transactionId: string) {
    this.dispatchEvent(
      new CustomEvent('offer-selected', {
        detail: { transactionId, composed: true, bubbles: true },
      })
    );
  }

  render() {
    if (!this.offers)
      return html`<mwc-circular-progress></mwc-circular-progress>`;

    return html`
      <mwc-list>
        ${this.offers.map(
          (offer) => html`
            <mwc-list-item @click=${() => this.offerSelected(offer.id)}>
              <div class="column">
                <span>
                  ${offer.transaction.debtor.id} =>
                  ${offer.transaction.creditor.id}
                </span>
                <span>
                  ${offer.transaction.amount}
                </span>
              </div>
            </mwc-list-item>
          `
        )}
      </mwc-list>
    `;
  }
}
