import { moduleConnect } from '@uprtcl/micro-orchestrator';
import { LitElement, html, property, css, PropertyValues } from 'lit-element';
import { sharedStyles } from './sharedStyles';
import { Offer } from 'src/types';
import { ApolloClientModule } from '@uprtcl/graphql';
import { ApolloClient, gql } from 'apollo-boost';
import {
  GET_OFFER_DETAIL,
  ACCEPT_OFFER,
  CANCEL_OFFER,
  GET_PENDING_OFFERS,
} from 'src/graphql/queries';
import { Agent } from 'holochain-profiles';

export class MCOfferDetail extends moduleConnect(LitElement) {
  @property({ type: String })
  transactionId!: string;

  @property({ type: String })
  myAgentId!: string;

  @property({ type: Object })
  offer!: Offer;

  @property({ type: Boolean })
  accepting: boolean = false;

  @property({ type: Boolean })
  canceling: boolean = false;

  client!: ApolloClient<any>;

  static get styles() {
    return sharedStyles;
  }

  updated(changedValues: PropertyValues) {
    super.updated(changedValues);

    if (changedValues.has('transactionId') && this.transactionId !== null) {
      this.loadOffer();
    }
  }

  async loadOffer() {
    this.offer = undefined as any;
    this.client = this.request(ApolloClientModule.bindings.Client);

    const result = await this.client.query({
      query: GET_OFFER_DETAIL,
      variables: {
        transactionId: this.transactionId,
      },
      fetchPolicy: 'network-only',
    });

    this.offer = result.data.offer;
    this.myAgentId = result.data.me.id;
  }

  acceptOffer() {
    this.accepting = true;

    this.client
      .mutate({
        mutation: ACCEPT_OFFER,
        variables: {
          transactionId: this.transactionId,
          approvedHeaderId: this.offer.counterpartySnapshot.lastHeaderId,
        },
      })
      .then(() => {
        this.dispatchEvent(
          new CustomEvent('offer-accepted', {
            detail: { transactionId: this.transactionId },
            composed: true,
            bubbles: true,
          })
        );
      })
      .finally(() => (this.accepting = false));
  }

  async cancelOffer() {
    (this.canceling = true),
      await this.client.mutate({
        mutation: CANCEL_OFFER,
        variables: {
          transactionId: this.transactionId,
        },
        update: (cache, result) => {
          const pendingOffers: any = cache.readQuery({
            query: GET_PENDING_OFFERS,
          });

          const offers = pendingOffers.myOffers.filter(
            (o) => o.id !== this.transactionId
          );

          pendingOffers.myOffers = offers;

          cache.writeQuery({ query: GET_PENDING_OFFERS, data: pendingOffers });
        },
      });

    this.dispatchEvent(
      new CustomEvent('offer-canceled', {
        detail: { transactionId: this.transactionId },
        bubbles: true,
        composed: true,
      })
    );
  }

  isOutgoing() {
    return this.offer.transaction.debtor.id === this.myAgentId;
  }

  getCounterparty(): Agent {
    return this.offer.transaction.creditor.id === this.myAgentId
      ? this.offer.transaction.debtor
      : this.offer.transaction.creditor;
  }

  renderCounterparty() {
    const cUsername = `@${this.getCounterparty().username}`;
    return html`
      <div class="row">
        <div class="column">
          <span class="item title">
            Offer ${this.isOutgoing() ? ' to ' : ' from '} ${cUsername}
          </span>
          <span class="item">Agend ID: ${this.getCounterparty().id}</span>

          <span class="item">
            Transaction amount: ${this.offer.transaction.amount} credits
          </span>
          <span class="item">
            Date:
            ${new Date(
              this.offer.transaction.timestamp * 1000
            ).toLocaleDateString()}
          </span>

          <span class="item title" style="margin-top: 8px;"
            >${cUsername} current status</span
          >

          <span class="item">
            Balance: ${this.offer.counterpartySnapshot.balance} credits
          </span>
          <span class="item">
            Transaction history is
            ${this.offer.counterpartySnapshot.valid ? 'valid' : 'invalid'}
          </span>
          <span class="item">
            Offer is ${this.offer.counterpartySnapshot.executable ? '' : 'not'}
            executable right now
          </span>
        </div>
      </div>
    `;
  }

  placeholderMessage() {
    if (this.accepting) return 'Accepting offer...';
    if (this.canceling) return 'Canceling offer...';
    return 'Fetching and verifying counterparty chain...';
  }

  render() {
    if (!this.offer || this.accepting || this.canceling)
      return html`<div class="column fill center-content">
        <mwc-circular-progress></mwc-circular-progress>
        <span style="margin-top: 18px;">${this.placeholderMessage()}</span>
      </div>`;

    return html`
      <div class="column">
        ${this.renderCounterparty()}
        <div class="row center-content" style="margin-top: 4px;">
          <mwc-button
            label="CANCEL"
            style="flex: 1; margin-right: 16px;"
            @click=${() => this.cancelOffer()}
          ></mwc-button>
          ${this.isOutgoing()
            ? html`<span style="flex: 1; opacity: 0.8;">
                Awaiting for approval
              </span>`
            : html`
                <mwc-button
                  style="flex: 1;"
                  .disabled=${!this.offer.counterpartySnapshot.executable ||
                  this.offer.state !== 'Pending'}
                  label="ACCEPT"
                  raised
                  @click=${() => this.acceptOffer()}
                ></mwc-button>
              `}
        </div>
      </div>
    `;
  }
}
