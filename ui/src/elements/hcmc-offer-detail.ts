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
  CONSENT_FOR_OFFER,
} from 'src/graphql/queries';
import { Agent } from 'holochain-profiles';
import { dateString } from 'src/utils';

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
  consenting: boolean = false;

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
    const loadingTransactionId = this.transactionId;
    this.offer = undefined as any;
    this.client = this.request(ApolloClientModule.bindings.Client);

    const result = await this.client.query({
      query: GET_OFFER_DETAIL,
      variables: {
        transactionId: this.transactionId,
      },
      fetchPolicy: 'network-only',
    });

    if (loadingTransactionId === this.transactionId) {
      this.offer = result.data.offer;
      this.myAgentId = result.data.me.id;
    }
  }

  acceptOffer() {
    if (!this.offer.counterparty.snapshot) return null;
    const transactionId = this.transactionId;

    this.accepting = true;

    this.client
      .mutate({
        mutation: ACCEPT_OFFER,
        variables: {
          transactionId,
          approvedHeaderId: this.offer.counterparty.snapshot.lastHeaderId,
        },
        update: (cache, result) => {
          const pendingOffers: any = cache.readQuery({
            query: GET_PENDING_OFFERS,
          });

          pendingOffers.me.offers.find((o) => o.id === transactionId).state =
            'Completed';

          cache.writeQuery({
            query: GET_PENDING_OFFERS,
            data: pendingOffers,
          });
        },
      })
      .then(() => {
        this.dispatchEvent(
          new CustomEvent('offer-accepted', {
            detail: { transactionId },
            composed: true,
            bubbles: true,
          })
        );
      })
      .catch(() => {
        this.dispatchEvent(
          new CustomEvent('offer-failed-to-accept', {
            detail: { transactionId },
            composed: true,
            bubbles: true,
          })
        );
        this.loadOffer();
      })
      .finally(() => (this.accepting = false));
  }

  consentOffer() {
    this.consenting = true;
    const transactionId = this.transactionId;

    this.client
      .mutate({
        mutation: CONSENT_FOR_OFFER,
        variables: {
          transactionId: this.transactionId,
        },
      })
      .then(() => {
        this.dispatchEvent(
          new CustomEvent('offer-consented', {
            detail: { transactionId },
            composed: true,
            bubbles: true,
          })
        );
        this.loadOffer();
      })
      .finally(() => (this.consenting = false));
  }

  async cancelOffer() {
    const transactionId = this.transactionId;
    (this.canceling = true),
      await this.client.mutate({
        mutation: CANCEL_OFFER,
        variables: {
          transactionId,
        },
        update: (cache, result) => {
          const pendingOffers: any = cache.readQuery({
            query: GET_PENDING_OFFERS,
          });

          const offers = pendingOffers.me.offers.filter(
            (o) => o.id !== transactionId
          );

          pendingOffers.me.offers = offers;

          cache.writeQuery({ query: GET_PENDING_OFFERS, data: pendingOffers });
        },
      });

    this.dispatchEvent(
      new CustomEvent('offer-canceled', {
        detail: { transactionId },
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

  getExecutableStatus(): string {
    if (!this.offer.counterparty.snapshot) return '';

    if (this.offer.counterparty.snapshot.executable) {
      if (this.isOutgoing())
        return `${this.getCounterpartyUsername()} can execute this offer right now`;
      else return `You can execute this offer right now`;
    } else return `Executing the offer would violate the credit limits`;
  }

  getCounterpartyUsername(): string {
    return `@${this.getCounterparty().username}`;
  }

  userShouldWait() {
    const snapshot = this.offer.counterparty.snapshot;
    return (
      snapshot &&
      !snapshot.valid &&
      snapshot.invalidReason.includes(
        'Number of attestations in the DHT does not match'
      )
    );
  }

  renderCounterpartyStatus() {
    if (!this.offer.counterparty.online)
      return html`
        <span class="item">
          ${this.getCounterpartyUsername()} is not online at the moment, cannot
          get their chain.
        </span>
      `;
    else if (!this.offer.counterparty.consented) {
      return html`
        <span class="item">
          ${this.offer.state !== 'Received'
            ? `${this.getCounterpartyUsername()} has `
            : 'You have '}
          not consented for to share
          ${this.offer.state !== 'Received' ? 'their' : 'your'} source chain yet
        </span>
      `;
    } else if (this.offer.counterparty.snapshot) {
      const balance = this.offer.counterparty.snapshot.balance;
      return html` <span class="item">
          Balance: ${balance > 0 ? '+' : ''}${balance} credits
        </span>
        ${this.userShouldWait()
          ? html`<span>
              Could not fetch last transaction of
              ${this.getCounterpartyUsername()} from the DHT: wait for eventual
              consistency and try again.
            </span>`
          : html`
              <span class="item">
                Transaction history is
                ${this.offer.counterparty.snapshot.valid
                  ? 'valid'
                  : 'invalid! You cannot transact with an invalid agent.'}
              </span>
            `}
        ${this.offer.counterparty.snapshot.valid
          ? html` <span class="item">${this.getExecutableStatus()} </span> `
          : html``}`;
    }
  }

  renderCounterparty() {
    return html`
      <div class="row">
        <div class="column">
          <span class="item title">
            Offer ${this.isOutgoing() ? ' to ' : ' from '}
            ${this.getCounterpartyUsername()}
          </span>
          <span class="item">Agend ID: ${this.getCounterparty().id}</span>

          <span class="item">
            Transaction amount: ${this.offer.transaction.amount} credits
          </span>
          <span class="item">
            Date: ${dateString(this.offer.transaction.timestamp)}
          </span>

          <span class="item title" style="margin-top: 16px;"
            >${this.getCounterpartyUsername()} current status</span
          >
          ${this.renderCounterpartyStatus()}
        </div>
      </div>
    `;
  }

  placeholderMessage() {
    if (this.accepting) return 'Accepting offer...';
    if (this.canceling) return 'Canceling offer...';
    if (this.consenting) return 'Consenting for offer...';
    return 'Fetching and verifying counterparty chain...';
  }

  getForwardActionLabel() {
    if (!this.offer.counterparty.online)
      return 'Awaiting for agent to be online';
    else if (!this.offer.counterparty.consented) return 'Awaiting for consent';
    else return 'Awaiting for approval';
  }

  renderOfferForwardAction() {
    if (this.isOutgoing() || !this.offer.counterparty.online)
      return html`<mwc-button
        style="flex: 1;"
        .label=${this.getForwardActionLabel()}
        disabled
        raised
      >
      </mwc-button>`;
    else if (this.offer.state == 'Received')
      return html`<mwc-button
        style="flex: 1;"
        label="CONSENT TO SHOW CHAIN"
        raised
        @click=${() => this.consentOffer()}
      ></mwc-button>`;
    else {
      const snapshot = this.offer.counterparty.snapshot;
      return html`
        <mwc-button
          style="flex: 1;"
          .disabled=${!(snapshot && snapshot.executable) ||
          this.offer.state !== 'Pending'}
          label="ACCEPT AND COMPLETE TRANSACTION"
          raised
          @click=${() => this.acceptOffer()}
        ></mwc-button>
      `;
    }
  }

  render() {
    if (!this.offer || this.accepting || this.canceling || this.consenting)
      return html`<div class="column fill center-content">
        <mwc-circular-progress></mwc-circular-progress>
        <span style="margin-top: 18px;" class="placeholder"
          >${this.placeholderMessage()}</span
        >
      </div>`;

    return html`
      <div class="column">
        ${this.renderCounterparty()}
        <div class="row center-content" style="margin-top: 24px;">
          <mwc-button
            .label=${(this.isOutgoing() ? 'CANCEL' : 'REJECT') + ' OFFER'}
            style="flex: 1; margin-right: 16px;"
            @click=${() => this.cancelOffer()}
          ></mwc-button>
          ${this.renderOfferForwardAction()}
        </div>
      </div>
    `;
  }
}
