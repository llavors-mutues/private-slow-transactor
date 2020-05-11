import { LitElement, html, property, css } from 'lit-element';
import { ApolloClient } from 'apollo-boost';
import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { ApolloClientModule } from '@uprtcl/graphql';

import '@material/mwc-list';
import '@authentic/mwc-circular-progress';

import { GET_PENDING_OFFERS } from '../graphql/queries';
import { Offer } from 'src/types';
import { sharedStyles } from './sharedStyles';
import { Agent } from 'holochain-profiles';
import { dateString } from 'src/utils';

export class MCPendingOfferList extends moduleConnect(LitElement) {
  client!: ApolloClient<any>;

  @property({ type: String })
  myAgentId!: string;

  @property({ type: Object, attribute: false })
  offers!: Offer[];

  @property({ type: String })
  lastSelectedOfferId: string | undefined = undefined;

  static get styles() {
    return [
      sharedStyles,
      css`
        :host {
          display: flex;
        }
      `,
    ];
  }

  async firstUpdated() {
    this.client = this.request(ApolloClientModule.bindings.Client);

    this.client
      .watchQuery({
        query: GET_PENDING_OFFERS,
        fetchPolicy: 'network-only',
      })
      .subscribe((result) => {
        this.myAgentId = result.data.me.id;
        this.offers = result.data.me.offers.filter(
          (offer) => offer.state !== 'Completed' && offer.state !== 'Canceled'
        );
      });
  }

  renderPlaceholder(type: string) {
    return html`<span style="padding-top: 16px;">
      You have no ${type.toLowerCase()} offers
    </span>`;
  }

  offerSelected(transactionId: string) {
    this.dispatchEvent(
      new CustomEvent('offer-selected', {
        detail: { transactionId, composed: true, bubbles: true },
      })
    );
    this.lastSelectedOfferId = transactionId;
  }

  isOutgoing(offer: Offer): boolean {
    return offer.transaction.debtor.id === this.myAgentId;
  }

  getOutgoing(): Offer[] {
    return this.offers.filter((offer) => this.isOutgoing(offer));
  }

  getIncoming(): Offer[] {
    return this.offers.filter((offer) => !this.isOutgoing(offer));
  }

  counterparty(offer: Offer): Agent {
    return offer.transaction.creditor.id === this.myAgentId
      ? offer.transaction.debtor
      : offer.transaction.creditor;
  }

  renderOfferList(title: string, offers: Offer[]) {
    return html`<div class="column">
      <span class="title">${title} offers</span>

      ${offers.length === 0
        ? this.renderPlaceholder(title)
        : html`
            <mwc-list>
              ${offers.map(
                (offer, index) => html`
                  <mwc-list-item
                    @click=${() => this.offerSelected(offer.id)}
                    graphic="avatar"
                    twoline
                    .activated=${this.lastSelectedOfferId &&
                    this.lastSelectedOfferId === offer.id}
                  >
                    <span>
                      ${offer.transaction.amount} credits
                      ${this.isOutgoing(offer) ? 'to' : 'from'}
                      @${this.counterparty(offer).username}
                    </span>
                    <span slot="secondary">
                      ${dateString(offer.transaction.timestamp)}
                    </span>
                    <mwc-icon
                      slot="graphic"
                      .style="color: ${this.isOutgoing(offer)
                        ? 'red'
                        : 'green'}"
                      >${this.isOutgoing(offer)
                        ? 'call_made'
                        : 'call_received'}</mwc-icon
                    >
                  </mwc-list-item>
                  ${index < offers.length - 1
                    ? html`<li divider padded role="separator"></li> `
                    : html``}
                `
              )}
            </mwc-list>
          `}
    </div>`;
  }

  render() {
    if (!this.offers)
      return html`<div class="column fill center-content">
        <mwc-circular-progress></mwc-circular-progress>
        <span class="placeholder" style="margin-top: 18px;"
          >Fetching pending offers...</span
        >
      </div>`;

    return html`<div class="column fill">
      <div style="margin-bottom: 24px;">
        ${this.renderOfferList('Incoming', this.getIncoming())}
      </div>
      ${this.renderOfferList('Outgoing', this.getOutgoing())}
    </div>`;
  }
}
