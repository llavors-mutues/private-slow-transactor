import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { LitElement, html, property } from 'lit-element';
import { sharedStyles } from './sharedStyles';
import { Offer } from 'src/types';
import { ApolloClientModule } from '@uprtcl/graphql';
import { ApolloClient } from 'apollo-boost';
import { GET_OFFER_DETAIL, ACCEPT_OFFER } from 'src/graphql/queries';

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

  acceptOffer() {
    this.client.mutate({
      mutation: ACCEPT_OFFER,
      variables: {
        transactionId: this.transactionId,
        approvedHeaderId: this.offer.counterpartySnapshot.lastHeaderId,
      },
    });
  }

  render() {
    if (!this.offer)
      return html`<mwc-circular-progress></mwc-circular-progress>`;

    return html`
      <div class="column">
        <span>${this.offer.counterpartySnapshot.balance}</span>
        <mwc-button
          label="ACCEPT"
          @click=${() => this.acceptOffer()}
        ></mwc-button>
      </div>
    `;
  }
}
