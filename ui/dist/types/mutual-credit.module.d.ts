import { interfaces } from 'inversify';
import { GraphQlSchemaModule } from '@uprtcl/graphql';
import { MicroModule, i18nextModule } from '@uprtcl/micro-orchestrator';
import { GetAllowedCreditors } from './types';
export declare class MutualCreditModule extends MicroModule {
    protected instance: string;
    protected agentFilter: GetAllowedCreditors;
    static id: string;
    dependencies: string[];
    static bindings: {
        MutualCreditProvider: string;
        ValidAgentFilter: string;
    };
    constructor(instance: string, agentFilter?: GetAllowedCreditors);
    onLoad(container: interfaces.Container): Promise<void>;
    get submodules(): (GraphQlSchemaModule | i18nextModule)[];
}
