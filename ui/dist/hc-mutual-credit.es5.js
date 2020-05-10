import { HolochainConnectionModule, createHolochainProvider } from '@uprtcl/holochain-provider';
import { ProfilesModule } from 'holochain-profiles';
import '@material/mwc-textfield';
import '@material/mwc-button';
import { TextFieldBase } from '@material/mwc-textfield/mwc-textfield-base';
import '@material/mwc-top-app-bar';
import '@material/mwc-list';
import '@authentic/mwc-circular-progress';
import gql from 'graphql-tag';
import { moduleConnect, MicroModule, i18nextModule } from '@uprtcl/micro-orchestrator';
import { css, LitElement, html, property, query } from 'lit-element';
import { gql as gql$1 } from 'apollo-boost';
import { ApolloClientModule, GraphQlSchemaModule } from '@uprtcl/graphql';

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

      counterpartySnapshot {
        executable
        valid
        balance
        lastHeaderId
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

const sharedStyles = css `
  .column {
    display: flex;
    flex-direction: column;
  }

  .row {
    display: flex;
    flex-direction: row;
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

class MCCreateOffer extends moduleConnect(LitElement) {
    constructor() {
        super(...arguments);
        this.open = false;
        this.creditor = undefined;
    }
    static get styles() {
        return sharedStyles;
    }
    firstUpdated() {
        this.client = this.request(ApolloClientModule.bindings.Client);
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
        return html `
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
          ></mwc-textfield>

          <mwc-textfield
            style="padding-top: 16px;"
            label="Amount"
            type="number"
            id="amount"
            min="0.1"
            step="0.1"
            autoValidate
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
    query('#amount'),
    __metadata("design:type", TextFieldBase)
], MCCreateOffer.prototype, "amountField", void 0);
__decorate([
    query('#creditor'),
    __metadata("design:type", TextFieldBase)
], MCCreateOffer.prototype, "creditorField", void 0);
__decorate([
    property({ type: Boolean }),
    __metadata("design:type", Boolean)
], MCCreateOffer.prototype, "open", void 0);
__decorate([
    property({ type: String }),
    __metadata("design:type", Object)
], MCCreateOffer.prototype, "creditor", void 0);

class MCPendingOfferList extends moduleConnect(LitElement) {
    static get styles() {
        return [
            sharedStyles,
            css `
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
            this.offers = result.data.me.offers.filter((offer) => offer.state !== 'Completed' && offer.state !== 'Canceled');
        });
    }
    renderPlaceholder(type) {
        return html `<span style="padding-top: 16px;">
      You have no ${type.toLowerCase()} offers
    </span>`;
    }
    offerSelected(transactionId) {
        this.dispatchEvent(new CustomEvent('offer-selected', {
            detail: { transactionId, composed: true, bubbles: true },
        }));
    }
    getOutgoing() {
        return this.offers.filter((offer) => offer.transaction.debtor.id === this.myAgentId);
    }
    getIncoming() {
        return this.offers.filter((offer) => offer.transaction.creditor.id === this.myAgentId);
    }
    counterparty(offer) {
        return offer.transaction.creditor.id === this.myAgentId
            ? offer.transaction.debtor
            : offer.transaction.creditor;
    }
    renderOfferList(title, offers) {
        return html `<div class="column " style="margin-bottom: 24px;">
      <span class="title">${title} offers</span>

      ${offers.length === 0
            ? this.renderPlaceholder(title)
            : html `
            <mwc-list>
              ${offers.map((offer) => html `
                  <mwc-list-item
                    @click=${() => this.offerSelected(offer.id)}
                    twoline
                  >
                    <span>
                      @${this.counterparty(offer).username}
                      ${offer.transaction.amount} credits
                    </span>
                    <span slot="secondary">
                      ${new Date(offer.transaction.timestamp * 1000).toLocaleDateString()}
                    </span>
                  </mwc-list-item>
                `)}
            </mwc-list>
          `}
    </div>`;
    }
    render() {
        if (!this.offers)
            return html `<div class="column fill center-content">
        <mwc-circular-progress></mwc-circular-progress>
      </div>`;
        return html `<div class="column fill">
      ${this.renderOfferList('Incoming', this.getIncoming())}
      ${this.renderOfferList('Outgoing', this.getOutgoing())}
    </div>`;
    }
}
__decorate([
    property({ type: String }),
    __metadata("design:type", String)
], MCPendingOfferList.prototype, "myAgentId", void 0);
__decorate([
    property({ type: Object, attribute: false }),
    __metadata("design:type", Array)
], MCPendingOfferList.prototype, "offers", void 0);

class MCTransactionList extends moduleConnect(LitElement) {
    static get styles() {
        return sharedStyles;
    }
    async firstUpdated() {
        const client = this.request(ApolloClientModule.bindings.Client);
        const result = await client.query({
            query: GET_MY_TRANSACTIONS,
            fetchPolicy: 'network-only',
        });
        this.myAgentId = result.data.me.id;
        this.transactions = result.data.me.transactions;
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
        return html `<div class="column center-content">
      ${this.renderContent()}
    </div>`;
    }
    renderContent() {
        if (!this.transactions)
            return html `
        <div class="padding center-content">
          <mwc-circular-progress></mwc-circular-progress>
        </div>
      `;
        if (this.transactions.length === 0)
            return html `<div class="padding">
        <span>You have no transactions in your history</span>
      </div>`;
        return html `
      <mwc-list style="width: 100%;">
        ${this.transactions.map((transaction, i) => html `
            <mwc-list-item twoline noninteractive>
              <span>
                ${this.isOutgoing(transaction) ? 'To ' : 'From '}
                @${this.getCounterparty(transaction).username}:
                ${`${this.isOutgoing(transaction) ? '-' : '+'}${transaction.amount}`}
                credits
              </span>
              <span slot="secondary"
                >${this.getCounterparty(transaction).id}) on
                ${new Date(transaction.timestamp * 1000).toLocaleDateString()}</span
              >
            </mwc-list-item>
            ${i < this.transactions.length - 1
            ? html `<li divider padded role="separator"></li> `
            : html ``}
          `)}
      </mwc-list>
    `;
    }
}
__decorate([
    property({ type: String }),
    __metadata("design:type", String)
], MCTransactionList.prototype, "myAgentId", void 0);
__decorate([
    property({ type: Object, attribute: false }),
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
    valid: Boolean!
    lastHeaderId: ID!
  }

  type Offer {
    id: ID!

    transaction: Transaction!

    counterpartySnapshot: CounterpartySnapshot

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
        async counterpartySnapshot(parent, _, { container }) {
            const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
            try {
                const snapshot = await mutualCreditProvider.call('get_counterparty_snapshot', {
                    transaction_address: parent.id,
                });
                return snapshot;
            }
            catch (e) {
                if (e.message.includes('Offer is not pending'))
                    return null;
            }
        },
    },
    CounterpartySnapshot: {
        lastHeaderId(parent) {
            return parent.last_header_address;
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

class MCOfferDetail extends moduleConnect(LitElement) {
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
        this.offer = undefined;
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
            this.dispatchEvent(new CustomEvent('offer-accepted', {
                detail: { transactionId: this.transactionId },
                composed: true,
                bubbles: true,
            }));
        })
            .finally(() => (this.accepting = false));
    }
    consentOffer() {
        this.consenting = true;
        this.client
            .mutate({
            mutation: CONSENT_FOR_OFFER,
            variables: {
                transactionId: this.transactionId,
            },
        })
            .then(() => {
            this.dispatchEvent(new CustomEvent('offer-consented', {
                detail: { transactionId: this.transactionId },
                composed: true,
                bubbles: true,
            }));
            this.loadOffer();
        })
            .finally(() => (this.consenting = false));
    }
    async cancelOffer() {
        (this.canceling = true),
            await this.client.mutate({
                mutation: CANCEL_OFFER,
                variables: {
                    transactionId: this.transactionId,
                },
                update: (cache, result) => {
                    const pendingOffers = cache.readQuery({
                        query: GET_PENDING_OFFERS,
                    });
                    const offers = pendingOffers.me.offers.filter((o) => o.id !== this.transactionId);
                    pendingOffers.me.offers = offers;
                    cache.writeQuery({ query: GET_PENDING_OFFERS, data: pendingOffers });
                },
            });
        this.dispatchEvent(new CustomEvent('offer-canceled', {
            detail: { transactionId: this.transactionId },
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
    renderCounterparty() {
        const cUsername = `@${this.getCounterparty().username}`;
        return html `
      <div class="row">
        <div class="column">
          <span class="item title">
            Offer ${this.isOutgoing() ? ' to ' : ' from '} ${cUsername}
          </span>
          <span class="item">Agend ID: ${this.getCounterparty().id}</span>

          <span class="item">
            Transaction amount:
            ${this.isOutgoing() ? '-' : '+'}${this.offer.transaction.amount}
            credits
          </span>
          <span class="item">
            Date:
            ${new Date(this.offer.transaction.timestamp * 1000).toLocaleTimeString()}
            on
            ${new Date(this.offer.transaction.timestamp * 1000).toLocaleDateString()}
          </span>

          <span class="item title" style="margin-top: 16px;"
            >${cUsername} current status</span
          >

          ${this.offer.counterpartySnapshot
            ? html `
                <span class="item">
                  Balance: ${this.offer.counterpartySnapshot.balance} credits
                </span>
                <span class="item">
                  Transaction history is
                  ${this.offer.counterpartySnapshot.valid ? 'valid' : 'invalid'}
                </span>
                <span class="item">
                  Offer is
                  ${this.offer.counterpartySnapshot.executable ? '' : 'not'}
                  executable right now
                </span>
              `
            : html `
                <span class="item">
                  ${this.offer.state !== 'Received'
                ? `${cUsername} has `
                : 'You have '}
                  not consented for to share
                  ${this.offer.state !== 'Received' ? 'their' : 'your'} source
                  chain yet
                </span>
              `}
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
    renderOfferForwardAction() {
        if (this.isOutgoing())
            return html `<mwc-button
        style="flex: 1;"
        .label="Awaiting for ${this.offer.counterpartySnapshot
                ? 'approval'
                : 'consent'}"
        disabled
        raised
      >
      </mwc-button>`;
        if (this.offer.state == 'Received')
            return html `<mwc-button
        style="flex: 1;"
        label="CONSENT TO SHOW CHAIN"
        raised
        @click=${() => this.consentOffer()}
      ></mwc-button>`;
        return html `
      <mwc-button
        style="flex: 1;"
        .disabled=${!this.offer.counterpartySnapshot.executable ||
            this.offer.state !== 'Pending'}
        label="ACCEPT"
        raised
        @click=${() => this.acceptOffer()}
      ></mwc-button>
    `;
    }
    render() {
        if (!this.offer || this.accepting || this.canceling || this.consenting)
            return html `<div class="column fill center-content">
        <mwc-circular-progress></mwc-circular-progress>
        <span style="margin-top: 18px;">${this.placeholderMessage()}</span>
      </div>`;
        return html `
      <div class="column">
        ${this.renderCounterparty()}
        <div class="row center-content" style="margin-top: 24px;">
          <mwc-button
            label="CANCEL"
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
    property({ type: String }),
    __metadata("design:type", String)
], MCOfferDetail.prototype, "transactionId", void 0);
__decorate([
    property({ type: String }),
    __metadata("design:type", String)
], MCOfferDetail.prototype, "myAgentId", void 0);
__decorate([
    property({ type: Object }),
    __metadata("design:type", Object)
], MCOfferDetail.prototype, "offer", void 0);
__decorate([
    property({ type: Boolean }),
    __metadata("design:type", Boolean)
], MCOfferDetail.prototype, "accepting", void 0);
__decorate([
    property({ type: Boolean }),
    __metadata("design:type", Boolean)
], MCOfferDetail.prototype, "consenting", void 0);
__decorate([
    property({ type: Boolean }),
    __metadata("design:type", Boolean)
], MCOfferDetail.prototype, "canceling", void 0);

const allAgentsAllowed = async (client) => {
    const result = await client.query({
        query: gql$1 `
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

class MCAllowedCreditorList extends moduleConnect(LitElement) {
    constructor() {
        super(...arguments);
        this.selectedCreditor = undefined;
        this.agents = undefined;
    }
    static get styles() {
        return sharedStyles;
    }
    async firstUpdated() {
        this.client = this.request(ApolloClientModule.bindings.Client);
        const getAllowedCreditors = this.request(MutualCreditBindings.ValidAgentFilter);
        const agents = await getAllowedCreditors(this.client);
        const result = await this.client.query({
            query: gql$1 `
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
        return html `
      <hcmc-create-offer
        id="create-offer-dialog"
        .creditor=${this.selectedCreditor}
        @offer-created=${() => (this.createOfferDialog.open = false)}
      >
      </hcmc-create-offer>
    `;
    }
    renderAgent(agent) {
        return html `
      <div class="row" style="align-items: center;">
        <mwc-list-item style="flex: 1;" twoline noninteractive>
          <span>${agent.username}</span>
          <span slot="secondary">${agent.id}</span>
        </mwc-list-item>

        <mwc-button
          label="Offer credits"
          style="padding-right: 16px;"
          outlined
          @click=${() => {
            this.selectedCreditor = agent;
            this.createOfferDialog.open = true;
        }}
        ></mwc-button>
      </div>
    `;
    }
    render() {
        return html `<div class="column center-content">
      ${this.renderContent()}
    </div>`;
    }
    renderContent() {
        if (!this.agents)
            return html `<div class="padding center-content">
        <mwc-circular-progress></mwc-circular-progress>
      </div>`;
        if (this.agents.length === 0)
            return html `<div class="padding">
        <span>There are no agents to which to offer credits</span>
      </div>`;
        return html `
      ${this.renderCreateOffer()}
      <mwc-list style="width: 100%;">
        ${this.agents.map((agent, i) => html `${this.renderAgent(agent)}
          ${this.agents && i < this.agents.length - 1
            ? html `<li divider padded role="separator"></li> `
            : html ``} `)}
      </mwc-list>
    `;
    }
}
__decorate([
    query('#create-offer-dialog'),
    __metadata("design:type", MCCreateOffer)
], MCAllowedCreditorList.prototype, "createOfferDialog", void 0);
__decorate([
    property({ type: Object }),
    __metadata("design:type", Object)
], MCAllowedCreditorList.prototype, "selectedCreditor", void 0);
__decorate([
    property({ type: Array }),
    __metadata("design:type", Object)
], MCAllowedCreditorList.prototype, "agents", void 0);

class MutualCreditModule extends MicroModule {
    constructor(instance, agentFilter = allAgentsAllowed) {
        super();
        this.instance = instance;
        this.agentFilter = agentFilter;
        this.dependencies = [HolochainConnectionModule.id, ProfilesModule.id];
    }
    async onLoad(container) {
        const mutualCreditProvider = createHolochainProvider(this.instance, 'transactor');
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
            new GraphQlSchemaModule(mutualCreditTypeDefs, resolvers),
            new i18nextModule('mutual-credit', { en: en }),
        ];
    }
}
MutualCreditModule.id = 'mutual-credit-module';
MutualCreditModule.bindings = MutualCreditBindings;

export { MutualCreditModule, ACCEPT_OFFER, CANCEL_OFFER, CREATE_OFFER, GET_MY_BALANCE, GET_MY_TRANSACTIONS, GET_OFFER_DETAIL, GET_PENDING_OFFERS };
//# sourceMappingURL=hc-mutual-credit.es5.js.map
