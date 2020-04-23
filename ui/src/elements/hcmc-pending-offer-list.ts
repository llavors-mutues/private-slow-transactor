import { LitElement, html, property } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { ApolloClientModule } from '@uprtcl/graphql';

import '@material/mwc-list';
import '@authentic/mwc-circular-progress';

import { GET_PENDING_OFFERS } from '../graphql/queries';
import { Transaction } from 'src/types';
import { sharedStyles } from './sharedStyles';

export class PendingOfferList extends moduleConnect(LitElement) {
  client!: ApolloClient<any>;

  @property({ type: Object, attribute: false })
  offers!: Transaction[];

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

  render() {
    if (!this.offers)
      return html`<mwc-circular-progress></mwc-circular-progress>`;

    return html`
      <mwc-list>
        ${this.offers.map(
          (offer) => html`
            <mwc-list-item>
              <div class="column">
                <span>
                  ${offer.debtor.id} => ${offer.creditor.id}
                </span>
                <span>
                  ${offer.amount}
                </span>
              </div>
            </mwc-list-item>
          `
        )}
      </mwc-list>
    `;
  }
}
