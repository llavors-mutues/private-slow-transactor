import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { LitElement, html, property } from 'lit-element';
import { sharedStyles } from './sharedStyles';
import { Offer } from 'src/types';
import { ApolloClientModule } from '@uprtcl/graphql';
import { ApolloClient } from 'apollo-boost';
import { GET_OFFER_DETAIL } from 'src/graphql/queries';

export class OfferDetail extends moduleConnect(LitElement) {
  @property({ type: String })
  transactionId!: string;

  @property({ type: Object })
  offer!: Offer;

  client!: ApolloClient<any>;

  static get styles() {
    return sharedStyles;
  }

  async firstUpdated() {
    this.client = this.request(ApolloClientModule.bindings.Client);

    const result = await this.client.query({
      query: GET_OFFER_DETAIL,
      variables: {
        transactionId: this.transactionId,
      },
    });

    this.offer = result.data.offer;
  }

  render() {
    if (!this.offer)
      return html`<mwc-circular-progress></mwc-circular-progress>`;

    return html` <span>${this.offer.counterpartySnapshot.balance}</span> `;
  }
}
