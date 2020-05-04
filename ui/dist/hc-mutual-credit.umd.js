(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports, require('@uprtcl/holochain-provider'), require('holochain-profiles'), require('@material/mwc-textfield'), require('@material/mwc-button'), require('@material/mwc-textfield/mwc-textfield-base'), require('@material/mwc-top-app-bar'), require('@material/mwc-list'), require('@authentic/mwc-circular-progress'), require('graphql-tag'), require('@uprtcl/micro-orchestrator'), require('lit-element'), require('apollo-boost'), require('@material/mwc-dialog'), require('@uprtcl/graphql')) :
    typeof define === 'function' && define.amd ? define(['exports', '@uprtcl/holochain-provider', 'holochain-profiles', '@material/mwc-textfield', '@material/mwc-button', '@material/mwc-textfield/mwc-textfield-base', '@material/mwc-top-app-bar', '@material/mwc-list', '@authentic/mwc-circular-progress', 'graphql-tag', '@uprtcl/micro-orchestrator', 'lit-element', 'apollo-boost', '@material/mwc-dialog', '@uprtcl/graphql'], factory) :
    (factory((global.hcMutualCredit = {}),global.holochainProvider,global.holochainProfiles,null,null,global.mwcTextfieldBase,null,null,null,global.gql,global.microOrchestrator,global.litElement,global.apolloBoost,global.mwcDialog,global.graphql));
}(this, (function (exports,holochainProvider,holochainProfiles,mwcTextfield,mwcButton,mwcTextfieldBase,mwcTopAppBar,mwcList,mwcCircularProgress,gql,microOrchestrator,litElement,apolloBoost,mwcDialog,graphql) { 'use strict';

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
    myBalance
  }
`;
    const GET_MY_TRANSACTIONS = gql `
  query GetMyTransactions {
    me {
      id
    }
    myTransactions {
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
`;
    const GET_PENDING_OFFERS = gql `
  query GetPendingOffers {
    me {
      id
    }
    myOffers {
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
    const ACCEPT_OFFER = gql `
  mutation AcceptOffer($transactionId: ID!, $approvedHeaderId: ID!) {
    acceptOffer(
      transactionId: $transactionId
      approvedHeaderId: $approvedHeaderId
    )
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

  .fill {
    flex: 1;
  }

  .center-content {
    justify-content: center;
    align-items: center;
  }

  .item {
    margin-bottom: 8px;
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
        }
        createOffer() {
            this.client.mutate({
                mutation: CREATE_OFFER,
                variables: {
                    creditorId: this.creditorField.value,
                    amount: parseFloat(this.amountField.value),
                },
            });
        }
        render() {
            return litElement.html `
      <div class="column center-content">
        <mwc-textfield
          style="padding: 16px 0;"
          label="Amount"
          type="number"
          id="amount"
          min="0.1"
          step="0.1"
          autoValidate
        ></mwc-textfield>

        <mwc-textfield
          .disabled=${this.creditor !== undefined}
          .value=${this.creditor}
          style="padding-bottom: 16px;"
          id="creditor"
          label="Creditor"
          autoValidate
        ></mwc-textfield>

        <mwc-button
          label="CREATE OFFER"
          raised
          @click=${() => this.createOffer()}
        ></mwc-button>
      </div>
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

    class MCPendingOfferList extends microOrchestrator.moduleConnect(litElement.LitElement) {
        static get styles() {
            return sharedStyles;
        }
        async firstUpdated() {
            this.client = this.request(graphql.ApolloClientModule.bindings.Client);
            const result = await this.client.query({
                query: GET_PENDING_OFFERS,
            });
            this.myAgentId = result.data.me.id;
            this.offers = result.data.myOffers.filter((o) => o.state !== 'Completed');
        }
        renderPlaceholder(type) {
            return litElement.html `<span style="padding-top: 16px;"
      >You have no ${type.toLowerCase()} offers</span
    >`;
        }
        offerSelected(transactionId) {
            this.dispatchEvent(new CustomEvent('offer-selected', {
                detail: { transactionId, composed: true, bubbles: true },
            }));
        }
        getPendingOffers() {
            return this.offers.filter((offer) => offer.state !== 'Completed');
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
            return litElement.html `<div class="column" style="margin-bottom: 24px;">
      <span class="title">${title} offers</span>

      ${offers.length === 0
            ? this.renderPlaceholder(title)
            : litElement.html `
            <mwc-list>
              ${offers.map((offer) => litElement.html `
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
                return litElement.html `<mwc-circular-progress></mwc-circular-progress>`;
            return litElement.html `<div class="column">
      ${this.renderOfferList('Incoming', this.getIncoming())}
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

    class MCTransactionList extends microOrchestrator.moduleConnect(litElement.LitElement) {
        async firstUpdated() {
            const client = this.request(graphql.ApolloClientModule.bindings.Client);
            const result = await client.query({
                query: GET_MY_TRANSACTIONS,
            });
            this.myAgentId = result.data.me.id;
            this.transactions = result.data.myTransactions;
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
                return litElement.html ` <mwc-circular-progress></mwc-circular-progress> `;
            if (this.transactions.length === 0)
                return litElement.html `<span>You have no transactions in your history</span>`;
            return litElement.html `
      <mwc-list>
        ${this.transactions.map((transaction) => litElement.html `
            <mwc-list-item twoline noninteractive>
              <span>
                ${this.isOutgoing(transaction) ? 'To ' : 'From '}
                @${this.getCounterparty(transaction).username}
                (${this.getCounterparty(transaction).id}):
                ${`${this.isOutgoing(transaction) ? '-' : '+'}${transaction.amount}`}
                credits
              </span>
              <span slot="secondary"
                >${new Date(transaction.timestamp * 1000).toLocaleDateString()}</span
              >
            </mwc-list-item>
            <mwc-list-divider></mwc-list-divider>
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
    Declined
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

  extend type Query {
    myTransactions: [Transaction!]!
    myOffers: [Offer!]!
    myBalance: Float!
    offer(transactionId: ID!): Offer!
  }

  extend type Mutation {
    createOffer(creditorId: ID!, amount: Float!): ID!
    declineOffer(transactionId: ID!): ID!
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
                return mutualCreditProvider.call('get_counterparty_snapshot', {
                    transaction_address: parent.id,
                });
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
            async myTransactions(_, __, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                const transactions = await mutualCreditProvider.call('query_my_transactions', {});
                return transactions.map((t) => ({ id: t[0], ...t[1] }));
            },
            async myOffers(_, __, { container }) {
                const mutualCreditProvider = container.get(MutualCreditBindings.MutualCreditProvider);
                const offers = await mutualCreditProvider.call('query_my_offers', {});
                console.log(offers);
                return offers.map((offer) => offerToTransaction(offer[0], offer[1]));
            },
            async myBalance(_, __, { container }) {
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
        },
    };

    class MCOfferDetail extends microOrchestrator.moduleConnect(litElement.LitElement) {
        static get styles() {
            return sharedStyles;
        }
        async firstUpdated() {
            this.client = this.request(graphql.ApolloClientModule.bindings.Client);
            const result = await this.client.query({
                query: GET_OFFER_DETAIL,
                variables: {
                    transactionId: this.transactionId,
                },
            });
            this.offer = result.data.offer;
            this.myAgentId = result.data.me.id;
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
            return litElement.html `
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
            ${new Date(this.offer.transaction.timestamp * 1000).toLocaleDateString()}
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
        render() {
            if (!this.offer)
                return litElement.html `<mwc-circular-progress></mwc-circular-progress>`;
            return litElement.html `
      <div class="column">
        ${this.renderCounterparty()}

        <div class="row" style="margin-top: 4px;">
          <mwc-button
            label="DECLINE"
            style="flex: 1;"
            @click=${() => this.acceptOffer()}
          ></mwc-button>
          <mwc-button
            style="flex: 1;"
            .disabled=${!this.offer.counterpartySnapshot.executable}
            label="ACCEPT"
            raised
            @click=${() => this.acceptOffer()}
          ></mwc-button>
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
            return litElement.html `<mwc-dialog id="create-offer-dialog">
      <hcmc-create-offer .creditor=${this.selectedCreditor}>
      </hcmc-create-offer>
    </mwc-dialog>`;
        }
        renderAgent(agent) {
            return litElement.html `
      <div class="row" style="align-items: center;">
        <mwc-list-item style="flex: 1;" twoline noninteractive>
          <span>${agent.username}</span>
          <span slot="secondary">${agent.id}</span>
        </mwc-list-item>

        <mwc-button
          label="Offer credits"
          style="padding-right: 16px;"
          @click=${() => {
            this.selectedCreditor = agent.id;
            this.createOfferDialog.open = true;
        }}
        ></mwc-button>
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
                return litElement.html `<mwc-circular-progress></mwc-circular-progress>`;
            if (this.agents.length === 0)
                return litElement.html `<span>There are no agents to which to offer credits</span>`;
            return litElement.html `
      ${this.renderCreateOffer()}
      <mwc-list>
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
        __metadata("design:type", mwcDialog.Dialog)
    ], MCAllowedCreditorList.prototype, "createOfferDialog", void 0);
    __decorate([
        litElement.property({ type: String }),
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

    Object.defineProperty(exports, '__esModule', { value: true });

})));
//# sourceMappingURL=hc-mutual-credit.umd.js.map
