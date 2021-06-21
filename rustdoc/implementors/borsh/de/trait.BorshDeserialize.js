(function() {var implementors = {};
implementors["anoma_shared"] = [{"text":"impl BorshDeserialize for <a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"anoma_shared/types/address/struct.EstablishedAddress.html\" title=\"struct anoma_shared::types::address::EstablishedAddress\">EstablishedAddress</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.ImplicitAddress.html\" title=\"enum anoma_shared::types::address::ImplicitAddress\">ImplicitAddress</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::address::Address"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/address/struct.EstablishedAddress.html\" title=\"struct anoma_shared::types::address::EstablishedAddress\">EstablishedAddress</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::address::EstablishedAddress"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/address/struct.EstablishedAddressGen.html\" title=\"struct anoma_shared::types::address::EstablishedAddressGen\">EstablishedAddressGen</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::address::EstablishedAddressGen"]},{"text":"impl BorshDeserialize for <a class=\"enum\" href=\"anoma_shared/types/address/enum.ImplicitAddress.html\" title=\"enum anoma_shared::types::address::ImplicitAddress\">ImplicitAddress</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.PublicKeyHash.html\" title=\"struct anoma_shared::types::key::ed25519::PublicKeyHash\">PublicKeyHash</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::address::ImplicitAddress"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/intent/struct.Intent.html\" title=\"struct anoma_shared::types::intent::Intent\">Intent</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"anoma_shared/types/token/struct.Amount.html\" title=\"struct anoma_shared::types::token::Amount\">Amount</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"anoma_shared/types/token/struct.Amount.html\" title=\"struct anoma_shared::types::token::Amount\">Amount</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::intent::Intent"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/intent/struct.IntentTransfers.html\" title=\"struct anoma_shared::types::intent::IntentTransfers\">IntentTransfers</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;<a class=\"struct\" href=\"anoma_shared/types/token/struct.Transfer.html\" title=\"struct anoma_shared::types::token::Transfer\">Transfer</a>&gt;: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>, <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.Signed.html\" title=\"struct anoma_shared::types::key::ed25519::Signed\">Signed</a>&lt;<a class=\"struct\" href=\"anoma_shared/types/intent/struct.Intent.html\" title=\"struct anoma_shared::types::intent::Intent\">Intent</a>&gt;&gt;: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::intent::IntentTransfers"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.PublicKeyHash.html\" title=\"struct anoma_shared::types::key::ed25519::PublicKeyHash\">PublicKeyHash</a>","synthetic":false,"types":["anoma_shared::types::key::ed25519::PublicKeyHash"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.SignedTxData.html\" title=\"struct anoma_shared::types::key::ed25519::SignedTxData\">SignedTxData</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;&gt;: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.Signature.html\" title=\"struct anoma_shared::types::key::ed25519::Signature\">Signature</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::key::ed25519::SignedTxData"]},{"text":"impl&lt;T:&nbsp;BorshSerialize + BorshDeserialize&gt; BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.Signed.html\" title=\"struct anoma_shared::types::key::ed25519::Signed\">Signed</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.Signature.html\" title=\"struct anoma_shared::types::key::ed25519::Signature\">Signature</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::key::ed25519::Signed"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.PublicKey.html\" title=\"struct anoma_shared::types::key::ed25519::PublicKey\">PublicKey</a>","synthetic":false,"types":["anoma_shared::types::key::ed25519::PublicKey"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.Signature.html\" title=\"struct anoma_shared::types::key::ed25519::Signature\">Signature</a>","synthetic":false,"types":["anoma_shared::types::key::ed25519::Signature"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/storage/struct.BlockHeight.html\" title=\"struct anoma_shared::types::storage::BlockHeight\">BlockHeight</a>","synthetic":false,"types":["anoma_shared::types::storage::BlockHeight"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/storage/struct.BlockHash.html\" title=\"struct anoma_shared::types::storage::BlockHash\">BlockHash</a>","synthetic":false,"types":["anoma_shared::types::storage::BlockHash"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/storage/struct.Key.html\" title=\"struct anoma_shared::types::storage::Key\">Key</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"enum\" href=\"anoma_shared/types/storage/enum.DbKeySeg.html\" title=\"enum anoma_shared::types::storage::DbKeySeg\">DbKeySeg</a>&gt;: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::storage::Key"]},{"text":"impl BorshDeserialize for <a class=\"enum\" href=\"anoma_shared/types/storage/enum.DbKeySeg.html\" title=\"enum anoma_shared::types::storage::DbKeySeg\">DbKeySeg</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::storage::DbKeySeg"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/token/struct.Amount.html\" title=\"struct anoma_shared::types::token::Amount\">Amount</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::token::Amount"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/token/struct.Transfer.html\" title=\"struct anoma_shared::types::token::Transfer\">Transfer</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"anoma_shared/types/token/struct.Amount.html\" title=\"struct anoma_shared::types::token::Amount\">Amount</a>: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::token::Transfer"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/transaction/struct.UpdateVp.html\" title=\"struct anoma_shared::types::transaction::UpdateVp\">UpdateVp</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::transaction::UpdateVp"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/types/validity_predicate/struct.EvalVp.html\" title=\"struct anoma_shared::types::validity_predicate::EvalVp\">EvalVp</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::types::validity_predicate::EvalVp"]},{"text":"impl BorshDeserialize for <a class=\"struct\" href=\"anoma_shared/vm/types/struct.KeyVal.html\" title=\"struct anoma_shared::vm::types::KeyVal\">KeyVal</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: BorshDeserialize,&nbsp;</span>","synthetic":false,"types":["anoma_shared::vm::types::KeyVal"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()