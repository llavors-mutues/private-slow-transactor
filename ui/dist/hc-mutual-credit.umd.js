(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports, require('@uprtcl/holochain-provider'), require('holochain-profiles'), require('@material/mwc-textfield'), require('@material/mwc-button'), require('@material/mwc-textfield/mwc-textfield-base'), require('@material/mwc-top-app-bar'), require('@material/mwc-list'), require('@authentic/mwc-circular-progress'), require('graphql-tag'), require('@uprtcl/micro-orchestrator'), require('lit-element'), require('apollo-boost'), require('@uprtcl/graphql')) :
    typeof define === 'function' && define.amd ? define(['exports', '@uprtcl/holochain-provider', 'holochain-profiles', '@material/mwc-textfield', '@material/mwc-button', '@material/mwc-textfield/mwc-textfield-base', '@material/mwc-top-app-bar', '@material/mwc-list', '@authentic/mwc-circular-progress', 'graphql-tag', '@uprtcl/micro-orchestrator', 'lit-element', 'apollo-boost', '@uprtcl/graphql'], factory) :
    (factory((global.hcMutualCredit = {}),global.holochainProvider,global.holochainProfiles,null,null,global.mwcTextfieldBase,null,null,null,global.gql,global.microOrchestrator,global.litElement,global.apolloBoost,global.graphql));
}(this, (function (exports,holochainProvider,holochainProfiles,mwcTextfield,mwcButton,mwcTextfieldBase,mwcTopAppBar,mwcList,mwcCircularProgress,gql,microOrchestrator,litElement,apolloBoost,graphql) { 'use strict';

    gql = gql && gql.hasOwnProperty('default') ? gql['default'] : gql;

    /*! *****************************************************************************
    Copyright (c) Microsoft Corporation. All rights reserved.
    Licensed under the Apache License, Version 2.0 (the "License"); you may not use
    this file except in compliance with the License. You may obtain a copy of the
    License at http://www.apache.org/licenses/LICENSE-2.0

    THIS CODE IS PROVIDED ON AN *AS IS* BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
    KIND, EITHER EXPRESS OR IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED
    WARRANTIES OR CONDITIONS OF TITLE, FITNESS FOR A PARTICULAR PURPOSE,
    MERCHANTABLITY OR NON-INFRINGEMENT.

    See the Apache Version 2.0 License for specific language governing permissions
    and limitations under the License.
    ***************************************************************************** */

    function __decorate(decorators, target, key, desc) {
        var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
        if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
        else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
        return c > 3 && r && Object.defineProperty(target, key, r), r;
    }

    function __metadata(metadataKey, metadataValue) {
        if (typeof Reflect === "object" && typeof Reflect.metadata === "function") return Reflect.metadata(metadataKey, metadataValue);
    }

    const GET_MY_BALANCE = gql `
  query GetMyBalance {
    me {
      id
      balance
    }
  }
`;
    const GET_MY_TRANSACTIONS = gql `
  query GetMyTransactions {
    me {
      id
      transactions {
        id
        debtor {
          id
          username
        }
        creditor {
          id
          username
        }
        amount
        timestamp
      }
    }
  }
`;
    const GET_PENDING_OFFERS = gql `
  query GetPendingOffers {
    me {
      id
      offers {
        id
        transaction {
          id
          debtor {
            id
            username
          }
          creditor {
            id
            username
          }
          amount
          timestamp
        }
        state
      }
    }
  }
`;
    const GET_OFFER_DETAIL = gql `
  query GetOfferDetail($transactionId: String!) {
    me {
      id
    }

    offer(transactionId: $transactionId) {
      id
      transaction {
        id
        debtor {
          id
          username
        }
        creditor {
          id
          username
        }
        amount
        timestamp
      }

      counterparty {
        online
        consented
        snapshot {
          executable
          valid
          invalidReason
          balance
          lastHeaderId
        }
      }

      state
    }
  }
`;
    const CREATE_OFFER = gql `
  mutation CreateOffer($creditorId: ID!, $amount: Float!) {
    createOffer(creditorId: $creditorId, amount: $amount)
  }
`;
    const CONSENT_FOR_OFFER = gql `
  mutation ConsentForOffer($transactionId: ID!) {
    consentForOffer(transactionId: $transactionId)
  }
`;
    const ACCEPT_OFFER = gql `
  mutation AcceptOffer($transactionId: ID!, $approvedHeaderId: ID!) {
    acceptOffer(
      transactionId: $transactionId
      approvedHeaderId: $approvedHeaderId
    )
  }
`;
    const CANCEL_OFFER = gql `
  mutation CancelOffer($transactionId: ID!) {
    cancelOffer(transactionId: $transactionId)
  }
`;

    const sharedStyles = litElement.css `
  .column {
    display: flex;
    flex-direction: column;
  }

  .row {
    display: flex;
    flex-direction: row;
  }

  .placeholder {
    opacity: 0.7;
  }

  .fill {
    flex: 1;
  }

  .center-content {
    justify-content: center;
    align-items: center;
  }

  .item {
    margin-bottom: 16px;
  }

  .padding {
    padding: 16px;
  }

  .title {
    font-weight: bold;
    font-size: 18px;
  }
`;

    class MCCreateOffer extends microOrchestrator.moduleConnect(litElement.LitElement) {
        constructor() {
            super(...arguments);
            this.open = false;
            this.creditor = undefined;
        }
        static get styles() {
            return sharedStyles;
        }
        firstUpdated() {
            this.client = this.request(graphql.ApolloClientModule.bindings.Client);
            this.amountField.validityTransform = (newValue) => {
                this.requestUpdate();
                try {
                    const amount = parseFloat(newValue);
                    if (amount > 0)
                        return { valid: true };
                }
                catch (e) { }
                this.amountField.setCustomValidity(`Offer amount has to be greater than 0`);
                return {
                    valid: false,
                };
            };
        }
        async createOffer() {
            const creditorId = this.creditorField.value;
            const amount = parseFloat(this.amountField.value);
            await this.client.mutate({
                mutation: CREATE_OFFER,
                variables: {
                    creditorId,
                    amount,
                },
            });
            this.dispatchEvent(new CustomEvent('offer-created', {
                detail: { creditorId, amount },
                composed: true,
                bubbles: true,
            }));
        }
        render() {
            return litElement.html `
      <mwc-dialog
        .open=${this.open}
        @closed=${() => (this.open = false)}
        heading="Create New Offer"
      >
        <div class="column center-content">
          <span>
            You are about to create an offer
            ${this.creditor ? `to @${this.creditor.username}` : ''}, which would
            lower your balance by the amount of the transaction and raise the
            creditor's value by the same amount.
            <br /><br />
            This will let the creditor scan your source chain to validate your
            transaction history.
          </span>
          <mwc-textfield
            .disabled=${this.creditor !== undefined}
            .value=${this.creditor && this.creditor.id}
            style="padding: 16px 0; width: 24em;"
            id="creditor"
            label="Creditor"
            autoValidate
            outlined
          ></mwc-textfield>

          <mwc-textfield
            style="padding-top: 16px;"
            label="Amount"
            type="number"
            id="amount"
            min="0.1"
            step="0.1"
            autoValidate
            outlined
          ></mwc-textfield>
        </div>

        <mwc-button slot="secondaryAction" dialogAction="cancel">
          Cancel
        </mwc-button>
        <mwc-button
          .disabled=${!this.amountField || !this.amountField.validity.valid}
          slot="primaryAction"
          @click=${() => this.createOffer()}
          dialogAction="create"
        >
          Create Offer
        </mwc-button>
      </mwc-dialog>
    `;
        }
    }
    __decorate([
        litElement.query('#amount'),
        __metadata("design:type", mwcTextfieldBase.TextFieldBase)
    ], MCCreateOffer.prototype, "amountField", void 0);
    __decorate([
        litElement.query('#creditor'),
        __metadata("design:type", mwcTextfieldBase.TextFieldBase)
    ], MCCreateOffer.prototype, "creditorField", void 0);
    __decorate([
        litElement.property({ type: Boolean }),
        __metadata("design:type", Boolean)
    ], MCCreateOffer.prototype, "open", void 0);
    __decorate([
        litElement.property({ type: String }),
        __metadata("design:type", Object)
    ], MCCreateOffer.prototype, "creditor", void 0);

    function dateString(timestamp) {
        return `${new Date(timestamp * 1000).toLocaleTimeString()}h,
                  ${new Date(timestamp * 1000).toDateString()}`;
    }

    class MCPendingOfferList extends microOrchestrator.moduleConnect(litElement.LitElement) {
        constructor() {
            super(...arguments);
            this.lastSelectedOfferId = undefined;
        }
        static get styles() {
            return [
                sharedStyles,
                litElement.css `
        :host {
          display: flex;
        }
      `,
            ];
        }
        async firstUpdated() {
            this.client = this.request(graphql.ApolloClientModule.bindings.Client);
            this.client
                .watchQuery({
                query: GET_PENDING_OFFERS,
                fetchPolicy: 'network-only',
            })
                .subscribe((result) => {
                this.myAgentId = result.data.me.id;
                this.offers = result.data.me.offers.filter((offer) => offer.state !== 'Completed' && offer.state !== 'Canceled');
            });
        }
        renderPlaceholder(type) {
            return litElement.html `<span style="padding-top: 16px;">
      You have no ${type.toLowerCase()} offers
    </span>`;
        }
        offerSelected(transactionId) {
            this.dispatchEvent(new CustomEvent('offer-selected', {
                detail: { transactionId, composed: true, bubbles: true },
            }));
            this.lastSelectedOfferId = transactionId;
        }
        isOutgoing(offer) {
            return offer.transaction.debtor.id === this.myAgentId;
        }
        getOutgoing() {
            return this.offers.filter((offer) => this.isOutgoing(offer));
        }
        getIncoming() {
            return this.offers.filter((offer) => !this.isOutgoing(offer));
        }
        counterparty(offer) {
            return offer.transaction.creditor.id === this.myAgentId
                ? offer.transaction.debtor
                : offer.transaction.creditor;
        }
        renderOfferList(title, offers) {
            return litElement.html `<div class="column">
      <span class="title">${title} offers</span>

      ${offers.length === 0
            ? this.renderPlaceholder(title)
            : litElement.html `
            <mwc-list>
              ${offers.map((offer, index) => litElement.html `
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
                ? litElement.html `<li divider padded role="separator"></li> `
                : litElement.html ``}
                `)}
            </mwc-list>
          `}
    </div>`;
        }
        render() {
            if (!this.offers)
                return litElement.html `<div class="column fill center-content">
        <mwc-circular-progress></mwc-circular-progress>
        <span class="placeholder" style="margin-top: 18px;"
          >Fetching pending offers...</span
        >
      </div>`;
            return litElement.html `<div class="column fill">
      <div style="margin-bottom: 24px;">
        ${this.renderOfferList('Incoming', this.getIncoming())}
      </div>
      ${this.renderOfferList('Outgoing', this.getOutgoing())}
    </div>`;
        }
    }
    __decorate([
        litElement.property({ type: String }),
        __metadata("design:type", String)
    ], MCPendingOfferList.prototype, "myAgentId", void 0);
    __decorate([
        litElement.property({ type: Object, attribute: false }),
        __metadata("design:type", Array)
    ], MCPendingOfferList.prototype, "offers", void 0);
    __decorate([
        litElement.property({ type: String }),
        __metadata("design:type", Object)
    ], MCPendingOfferList.prototype, "lastSelectedOfferId", void 0);

    class MCTransactionList extends microOrchestrator.moduleConnect(litElement.LitElement) {
        static get styles() {
            return sharedStyles;
        }
        async firstUpdated() {
            const client = this.request(graphql.ApolloClientModule.bindings.Client);
            const result = await client.query({
                query: GET_MY_TRANSACTIONS,
                fetchPolicy: 'network-only',
            });
            this.myAgentId = result.data.me.id;
            this.transactions = result.data.me.transactions.sort((t1, t2) => t2.timestamp - t1.timestamp);
        }
        isOutgoing(transaction) {
            return transaction.debtor.id === this.myAgentId;
        }
        getCounterparty(transaction) {
            return transaction.creditor.id === this.myAgentId
                ? transaction.debtor
                : transaction.creditor;
        }
        render() {
            return litElement.html `<div class="column center-content">
      ${this.renderContent()}
    </div>`;
        }
        renderContent() {
            if (!this.transactions)
                return litElement.html `
        <div class="padding center-content column">
          <mwc-circular-progress></mwc-circular-progress>
          <span class="placeholder" style="margin-top: 18px;"
            >Fetching transaction history...</span
          >
        </div>
      `;
            if (this.transactions.length === 0)
                return litElement.html `<div class="padding">
        <span>You have no transactions in your history</span>
      </div>`;
            return litElement.html `
      <mwc-list style="width: 100%;">
        ${this.transactions.map((transaction, i) => litElement.html `
            <div class="row" style="align-items: center;">
              <mwc-list-item
                twoline
                noninteractive
                graphic="avatar"
                style="flex: 1;"
              >
                <span>
                  ${this.isOutgoing(transaction) ? 'To ' : 'From '}
                  @${this.getCounterparty(transaction).username} on
                  ${dateString(transaction.timestamp)}
                </span>
                <span slot="secondary"
                  >${this.getCounterparty(transaction).id}
                </span>
                <mwc-icon
                  slot="graphic"
                  .style="color: ${this.isOutgoing(transaction)
            ? 'red'
            : 'green'}"
                  >${this.isOutgoing(transaction)
            ? 'call_made'
            : 'call_received'}</mwc-icon
                >
              </mwc-list-item>

              <span style="font-size: 20px; margin-right: 24px;">
                ${this.isOutgoing(transaction) ? '-' : '+'}${transaction.amount}
                credits
              </span>
            </div>
            ${i < this.transactions.length - 1
            ? litElement.html `<li divider padded role="separator"></li> `
            : litElement.html ``}
          `)}
      </mwc-list>
    `;
        }
    }
    __decorate([
        litElement.property({ type: String }),
        __metadata("design:type", String)
    ], MCTransactionList.prototype, "myAgentId", void 0);
    __decorate([
        litElement.property({ type: Object, attribute: false }),
        __metadata("design:type", Array)
    ], MCTransactionList.prototype, "transactions", void 0);

    var en = {
    	
    };

    const mutualCreditTypeDefs = gql `
  scalar Date

  enum OfferState {
    Received
    Pending
    Canceled
    Approved
    Completed
  }

  type Transaction {
    id: ID!

    debtor: Agent!
    creditor: Agent!
    amount: Float!
    timestamp: Date!
  }

  type CounterpartySnapshot {
    executable: Boolean!
    balance: Float!
    invalidReason: String
    valid: Boolean!
    lastHeaderId: ID!
  }

  type Counterparty {
    online: Boolean!
    consented: Boolean
    snapshot: CounterpartySnapshot
  }

  type Offer {
    id: ID!

    transaction: Transaction!

    counterparty: Counterparty!

    state: OfferState!
  }

  extend type Me {
    transactions: [Transaction!]!
    offers: [Offer!]!
    balance: Float!
  }

  extend type Query {
    offer(transactionId: ID!): Offer!
  }

  extend type Mutation {
    createOffer(creditorId: ID!, amount: Float!): ID!
    consentForOffer(transactionId: ID!): ID!
    cancelOffer(transactionId: ID!): ID!
    acceptOffer(transactionId: ID!, approvedHeaderId: ID!): ID!
  }
`;

    const MutualCreditBindings = {
        MutualCreditProvider: 'mutual-credit-provider',
        ValidAgentFilter: 'valid-agent-filter',
    };

    function offerToTransaction(id, offer) {
        const state = offer.state;
        return {
            id,
            transaction: {
                id,
                ...offer.transaction,
            },
            state: typeof state === 'object' ? Object.keys(state)[0] : state,
        };
    }
    const resolvers = {
        Transaction: {
            creditor(parent) {
                return { id: parent.creditor_address };
            },
            debtor(parent) {
                return { id: parent.debtor_address };
            },
        },
        Offer: {
            async counterparty(parent, _, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                try {
                    const snapshot = await mutualCreditProvider.call('get_counterparty_snapshot', {
                        transaction_address: parent.id,
                    });
                    return {
                        online: true,
                        consented: true,
                        snapshot,
                    };
                }
                catch (e) {
                    if (e.message.includes('Offer is not pending')) {
                        return {
                            online: true,
                            consented: false,
                            snapshot: null,
                        };
                    }
                    else if (e.message.includes('Counterparty is offline')) {
                        return {
                            online: false,
                            consented: null,
                            snapshot: null,
                        };
                    }
                }
            },
        },
        CounterpartySnapshot: {
            lastHeaderId(parent) {
                return parent.last_header_address;
            },
            invalidReason(parent) {
                return parent.invalid_reason;
            },
        },
        Query: {
            async offer(_, { transactionId }, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                const offer = await mutualCreditProvider.call('query_offer', {
                    transaction_address: transactionId,
                });
                return offerToTransaction(transactionId, offer);
            },
        },
        Me: {
            async transactions(_, __, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                const transactions = await mutualCreditProvider.call('query_my_transactions', {});
                return transactions.map((t) => ({ id: t[0], ...t[1] }));
            },
            async offers(_, __, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                const offers = await mutualCreditProvider.call('query_my_offers', {});
                console.log(offers);
                return offers.map((offer) => offerToTransaction(offer[0], offer[1]));
            },
            async balance(_, __, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                const result = await mutualCreditProvider.call('query_my_balance', {});
                return result.hasOwnProperty('Ok') ? result.Ok : result;
            },
        },
        Mutation: {
            async createOffer(_, { creditorId, amount }, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                return mutualCreditProvider.call('create_offer', {
                    creditor_address: creditorId,
                    amount,
                    timestamp: Math.floor(Date.now() / 1000),
                });
            },
            async acceptOffer(_, { transactionId, approvedHeaderId }, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                await mutualCreditProvider.call('accept_offer', {
                    transaction_address: transactionId,
                    approved_header_address: approvedHeaderId,
                });
                return transactionId;
            },
            async consentForOffer(_, { transactionId }, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                await mutualCreditProvider.call('consent_for_offer', {
                    transaction_address: transactionId,
                });
                return transactionId;
            },
            async cancelOffer(_, { transactionId }, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                await mutualCreditProvider.call('cancel_offer', {
                    transaction_address: transactionId,
                });
                return transactionId;
            },
        },
    };

    class MCOfferDetail extends microOrchestrator.moduleConnect(litElement.LitElement) {
        constructor() {
            super(...arguments);
            this.accepting = false;
            this.consenting = false;
            this.canceling = false;
        }
        static get styles() {
            return sharedStyles;
        }
        updated(changedValues) {
            super.updated(changedValues);
            if (changedValues.has('transactionId') && this.transactionId !== null) {
                this.loadOffer();
            }
        }
        async loadOffer() {
            const loadingTransactionId = this.transactionId;
            this.offer = undefined;
            this.client = this.request(graphql.ApolloClientModule.bindings.Client);
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
            if (!this.offer.counterparty.snapshot)
                return null;
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
                    const pendingOffers = cache.readQuery({
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
                this.dispatchEvent(new CustomEvent('offer-accepted', {
                    detail: { transactionId },
                    composed: true,
                    bubbles: true,
                }));
            })
                .catch(() => {
                this.dispatchEvent(new CustomEvent('offer-failed-to-accept', {
                    detail: { transactionId },
                    composed: true,
                    bubbles: true,
                }));
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
                this.dispatchEvent(new CustomEvent('offer-consented', {
                    detail: { transactionId },
                    composed: true,
                    bubbles: true,
                }));
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
                        const pendingOffers = cache.readQuery({
                            query: GET_PENDING_OFFERS,
                        });
                        const offers = pendingOffers.me.offers.filter((o) => o.id !== transactionId);
                        pendingOffers.me.offers = offers;
                        cache.writeQuery({ query: GET_PENDING_OFFERS, data: pendingOffers });
                    },
                });
            this.dispatchEvent(new CustomEvent('offer-canceled', {
                detail: { transactionId },
                bubbles: true,
                composed: true,
            }));
        }
        isOutgoing() {
            return this.offer.transaction.debtor.id === this.myAgentId;
        }
        getCounterparty() {
            return this.offer.transaction.creditor.id === this.myAgentId
                ? this.offer.transaction.debtor
                : this.offer.transaction.creditor;
        }
        getExecutableStatus() {
            if (!this.offer.counterparty.snapshot)
                return '';
            if (this.offer.counterparty.snapshot.executable) {
                if (this.isOutgoing())
                    return `${this.getCounterpartyUsername()} can execute this offer right now`;
                else
                    return `You can execute this offer right now`;
            }
            else
                return `Executing the offer would violate the credit limits`;
        }
        getCounterpartyUsername() {
            return `@${this.getCounterparty().username}`;
        }
        userShouldWait() {
            const snapshot = this.offer.counterparty.snapshot;
            return (snapshot &&
                !snapshot.valid &&
                snapshot.invalidReason.includes('Number of attestations in the DHT does not match'));
        }
        renderCounterpartyStatus() {
            if (!this.offer.counterparty.online)
                return litElement.html `
        <span class="item">
          ${this.getCounterpartyUsername()} is not online at the moment, cannot
          get their chain.
        </span>
      `;
            else if (!this.offer.counterparty.consented) {
                return litElement.html `
        <span class="item">
          ${this.offer.state !== 'Received'
                ? `${this.getCounterpartyUsername()} has `
                : 'You have '}
          not consented for to share
          ${this.offer.state !== 'Received' ? 'their' : 'your'} source chain yet
        </span>
      `;
            }
            else if (this.offer.counterparty.snapshot) {
                const balance = this.offer.counterparty.snapshot.balance;
                return litElement.html ` <span class="item">
          Balance: ${balance > 0 ? '+' : ''}${balance} credits
        </span>
        ${this.userShouldWait()
                ? litElement.html `<span>
              Could not fetch last transaction of
              ${this.getCounterpartyUsername()} from the DHT: wait for eventual
              consistency and try again.
            </span>`
                : litElement.html `
              <span class="item">
                Transaction history is
                ${this.offer.counterparty.snapshot.valid
                    ? 'valid'
                    : 'invalid! You cannot transact with an invalid agent.'}
              </span>
            `}
        ${this.offer.counterparty.snapshot.valid
                ? litElement.html ` <span class="item">${this.getExecutableStatus()} </span> `
                : litElement.html ``}`;
            }
        }
        renderCounterparty() {
            return litElement.html `
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
            if (this.accepting)
                return 'Accepting offer...';
            if (this.canceling)
                return 'Canceling offer...';
            if (this.consenting)
                return 'Consenting for offer...';
            return 'Fetching and verifying counterparty chain...';
        }
        getForwardActionLabel() {
            if (!this.offer.counterparty.online)
                return 'Awaiting for agent to be online';
            else if (!this.offer.counterparty.consented)
                return 'Awaiting for consent';
            else
                return 'Awaiting for approval';
        }
        renderOfferForwardAction() {
            if (this.isOutgoing() || !this.offer.counterparty.online)
                return litElement.html `<mwc-button
        style="flex: 1;"
        .label=${this.getForwardActionLabel()}
        disabled
        raised
      >
      </mwc-button>`;
            else if (this.offer.state == 'Received')
                return litElement.html `<mwc-button
        style="flex: 1;"
        label="CONSENT TO SHOW CHAIN"
        raised
        @click=${() => this.consentOffer()}
      ></mwc-button>`;
            else {
                const snapshot = this.offer.counterparty.snapshot;
                return litElement.html `
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
                return litElement.html `<div class="column fill center-content">
        <mwc-circular-progress></mwc-circular-progress>
        <span style="margin-top: 18px;" class="placeholder"
          >${this.placeholderMessage()}</span
        >
      </div>`;
            return litElement.html `
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
    __decorate([
        litElement.property({ type: String }),
        __metadata("design:type", String)
    ], MCOfferDetail.prototype, "transactionId", void 0);
    __decorate([
        litElement.property({ type: String }),
        __metadata("design:type", String)
    ], MCOfferDetail.prototype, "myAgentId", void 0);
    __decorate([
        litElement.property({ type: Object }),
        __metadata("design:type", Object)
    ], MCOfferDetail.prototype, "offer", void 0);
    __decorate([
        litElement.property({ type: Boolean }),
        __metadata("design:type", Boolean)
    ], MCOfferDetail.prototype, "accepting", void 0);
    __decorate([
        litElement.property({ type: Boolean }),
        __metadata("design:type", Boolean)
    ], MCOfferDetail.prototype, "consenting", void 0);
    __decorate([
        litElement.property({ type: Boolean }),
        __metadata("design:type", Boolean)
    ], MCOfferDetail.prototype, "canceling", void 0);

    const allAgentsAllowed = async (client) => {
        const result = await client.query({
            query: apolloBoost.gql `
      {
        allAgents {
          id
          username
        }
      }
    `,
        });
        return result.data.allAgents;
    };

    class MCAllowedCreditorList extends microOrchestrator.moduleConnect(litElement.LitElement) {
        constructor() {
            super(...arguments);
            this.selectedCreditor = undefined;
            this.agents = undefined;
        }
        static get styles() {
            return sharedStyles;
        }
        async firstUpdated() {
            this.client = this.request(graphql.ApolloClientModule.bindings.Client);
            const getAllowedCreditors = this.request(MutualCreditBindings.ValidAgentFilter);
            const agents = await getAllowedCreditors(this.client);
            const result = await this.client.query({
                query: apolloBoost.gql `
        {
          me {
            id
          }
        }
      `,
            });
            this.agents = agents.filter((a) => a.id !== result.data.me.id);
        }
        renderCreateOffer() {
            return litElement.html `
      <hcmc-create-offer
        id="create-offer-dialog"
        .creditor=${this.selectedCreditor}
        @offer-created=${() => (this.createOfferDialog.open = false)}
      >
      </hcmc-create-offer>
    `;
        }
        renderAgent(agent) {
            return litElement.html `
      <div class="row" style="align-items: center;">
        <mwc-list-item style="flex: 1;" twoline noninteractive graphic="avatar">
          <span>@${agent.username}</span>
          <span slot="secondary">${agent.id}</span>
          <mwc-icon slot="graphic">person</mwc-icon>
        </mwc-list-item>

        <mwc-button
          label="Offer credits"
          style="padding-right: 16px;"
          outlined
          @click=${() => {
            this.selectedCreditor = agent;
            this.createOfferDialog.open = true;
        }}
        >
          <mwc-icon style="padding-top: 3px;" slot="trailingIcon">send</mwc-icon>
        </mwc-button>
      </div>
    `;
        }
        render() {
            return litElement.html `<div class="column center-content">
      ${this.renderContent()}
    </div>`;
        }
        renderContent() {
            if (!this.agents)
                return litElement.html `<div class="padding center-content">
        <mwc-circular-progress></mwc-circular-progress>
      </div>`;
            if (this.agents.length === 0)
                return litElement.html `<div class="padding">
        <span>There are no agents to which to offer credits</span>
      </div>`;
            return litElement.html `
      ${this.renderCreateOffer()}
      <mwc-list style="width: 100%;">
        ${this.agents.map((agent, i) => litElement.html `${this.renderAgent(agent)}
          ${this.agents && i < this.agents.length - 1
            ? litElement.html `<li divider padded role="separator"></li> `
            : litElement.html ``} `)}
      </mwc-list>
    `;
        }
    }
    __decorate([
        litElement.query('#create-offer-dialog'),
        __metadata("design:type", MCCreateOffer)
    ], MCAllowedCreditorList.prototype, "createOfferDialog", void 0);
    __decorate([
        litElement.property({ type: Object }),
        __metadata("design:type", Object)
    ], MCAllowedCreditorList.prototype, "selectedCreditor", void 0);
    __decorate([
        litElement.property({ type: Array }),
        __metadata("design:type", Object)
    ], MCAllowedCreditorList.prototype, "agents", void 0);

    class MutualCreditModule extends microOrchestrator.MicroModule {
        constructor(instance, agentFilter = allAgentsAllowed) {
            super();
            this.instance = instance;
            this.agentFilter = agentFilter;
            this.dependencies = [holochainProvider.HolochainConnectionModule.id, holochainProfiles.ProfilesModule.id];
        }
        async onLoad(container) {
            const mutualCreditProvider = holochainProvider.createHolochainProvider(this.instance, 'transactor');
            container
                .bind(MutualCreditBindings.MutualCreditProvider)
                .to(mutualCreditProvider);
            container
                .bind(MutualCreditBindings.ValidAgentFilter)
                .toConstantValue(this.agentFilter);
            customElements.define('hcmc-transaction-list', MCTransactionList);
            customElements.define('hcmc-create-offer', MCCreateOffer);
            customElements.define('hcmc-pending-offer-list', MCPendingOfferList);
            customElements.define('hcmc-offer-detail', MCOfferDetail);
            customElements.define('hcmc-allowed-creditor-list', MCAllowedCreditorList);
        }
        get submodules() {
            return [
                new graphql.GraphQlSchemaModule(mutualCreditTypeDefs, resolvers),
                new microOrchestrator.i18nextModule('mutual-credit', { en: en }),
            ];
        }
    }
    MutualCreditModule.id = 'mutual-credit-module';
    MutualCreditModule.bindings = MutualCreditBindings;

    exports.MutualCreditModule = MutualCreditModule;
    exports.ACCEPT_OFFER = ACCEPT_OFFER;
    exports.CANCEL_OFFER = CANCEL_OFFER;
    exports.CREATE_OFFER = CREATE_OFFER;
    exports.GET_MY_BALANCE = GET_MY_BALANCE;
    exports.GET_MY_TRANSACTIONS = GET_MY_TRANSACTIONS;
    exports.GET_OFFER_DETAIL = GET_OFFER_DETAIL;
    exports.GET_PENDING_OFFERS = GET_PENDING_OFFERS;

    Object.defineProperty(exports, '__esModule', { value: true });

})));
//# sourceMappingURL=hc-mutual-credit.umd.js.map
