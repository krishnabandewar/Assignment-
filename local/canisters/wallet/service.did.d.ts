import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface _SERVICE {
  'check_balance' : ActorMethod<[string], bigint>,
  'init_balance' : ActorMethod<
    [bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'receive_token' : ActorMethod<
    [string, bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
  'send_token' : ActorMethod<
    [string, bigint],
    { 'Ok' : string } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
