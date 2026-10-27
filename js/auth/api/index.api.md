# `@moq/auth`

```ts
export type Algorithm = z.infer<typeof AlgorithmSchema>;

export const AlgorithmSchema: z.ZodMiniEnum<{
	ES256: "ES256";
	ES384: "ES384";
	EdDSA: "EdDSA";
	HS256: "HS256";
	HS384: "HS384";
	HS512: "HS512";
	PS256: "PS256";
	PS384: "PS384";
	PS512: "PS512";
	RS256: "RS256";
	RS384: "RS384";
	RS512: "RS512";
}>;

export type AsymmetricKey = Exclude<Key, {
	kty: "oct";
}>;

export type Bytes = z.infer<typeof BytesSchema>;

export const BytesSchema: z.ZodMiniObject<{
	received: z.ZodMiniInt;
	sent: z.ZodMiniInt;
}, z.core.$strip>;

export type Claims = z.output<typeof ClaimsSchema>;

export const ClaimsSchema: z.ZodMiniPipe<z.ZodMiniObject<{
	exp: z.ZodMiniOptional<z.ZodMiniInt>;
	get: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniString<string>, z.ZodMiniArray<z.ZodMiniString<string>>]>>;
	iat: z.ZodMiniOptional<z.ZodMiniInt>;
	iss: z.ZodMiniOptional<z.ZodMiniUnknown>;
	jti: z.ZodMiniOptional<z.ZodMiniUnknown>;
	nbf: z.ZodMiniOptional<z.ZodMiniInt>;
	publish: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<string>>>;
	put: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniString<string>, z.ZodMiniArray<z.ZodMiniString<string>>]>>;
	root: z.ZodMiniDefault<z.ZodMiniString<string>>;
	sub: z.ZodMiniOptional<z.ZodMiniUnknown>;
	subscribe: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<string>>>;
}, z.core.$strict>, z.ZodMiniTransform<Omit<{
	exp?: number;
	get?: string | string[];
	iat?: number;
	nbf?: number;
	publish?: string[];
	put?: string | string[];
	root: string;
	subscribe?: string[];
}, "get" | "put"> & {
	publish?: string[];
	subscribe?: string[];
}, {
	exp?: number;
	get?: string | string[];
	iat?: number;
	iss?: unknown;
	jti?: unknown;
	nbf?: number;
	publish?: string[];
	put?: string | string[];
	root: string;
	sub?: unknown;
	subscribe?: string[];
}>>;

export type Grant = z.infer<typeof GrantSchema>;

export const GrantSchema: z.ZodMiniObject<{
	expires: z.ZodMiniOptional<z.ZodMiniInt>;
	label: z.ZodMiniOptional<z.ZodMiniString<string>>;
	mounts: z.ZodMiniOptional<z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniString<string>>>;
	peer: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
	publish: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<string>>>;
	revalidate: z.ZodMiniOptional<z.ZodMiniInt>;
	root: z.ZodMiniOptional<z.ZodMiniString<string>>;
	subscribe: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<string>>>;
	tier: z.ZodMiniOptional<z.ZodMiniString<string>>;
	upstream: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
}, z.core.$strip>;

export { InvalidPattern } from "@moq/pattern";

export const Key: {
	generate: (algorithm: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512", kid?: string) => Promise<{
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		k: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "oct";
		scope?: Omit<{
			get?: ...[];
			publish?: ...[];
			put?: ...[];
			root: string;
			subscribe?: ...[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		d?: string;
		dp?: string;
		dq?: string;
		e: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "RSA";
		n: string;
		p?: string;
		q?: string;
		qi?: string;
		scope?: Omit<{
			get?: ...[];
			publish?: ...[];
			put?: ...[];
			root: string;
			subscribe?: ...[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "P-256" | "P-384";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "EC";
		scope?: Omit<{
			get?: ...[];
			publish?: ...[];
			put?: ...[];
			root: string;
			subscribe?: ...[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
		y: string;
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "Ed25519";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "OKP";
		scope?: Omit<{
			get?: ...[];
			publish?: ...[];
			put?: ...[];
			root: string;
			subscribe?: ...[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
	}>;
	parse: (jwk: string) => {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		k: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "oct";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		d?: string;
		dp?: string;
		dq?: string;
		e: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "RSA";
		n: string;
		p?: string;
		q?: string;
		qi?: string;
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "P-256" | "P-384";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "EC";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
		y: string;
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "Ed25519";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "OKP";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
	};
	public: (key: {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		k: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "oct";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		d?: string;
		dp?: string;
		dq?: string;
		e: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "RSA";
		n: string;
		p?: string;
		q?: string;
		qi?: string;
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "P-256" | "P-384";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "EC";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
		y: string;
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "Ed25519";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "OKP";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
	}) => PublicKey;
	sign: (key: {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		k: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "oct";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		d?: string;
		dp?: string;
		dq?: string;
		e: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "RSA";
		n: string;
		p?: string;
		q?: string;
		qi?: string;
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "P-256" | "P-384";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "EC";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
		y: string;
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "Ed25519";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "OKP";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
	}, claims: {
		exp?: number;
		get?: string | string[];
		iat?: number;
		iss?: unknown;
		jti?: unknown;
		nbf?: number;
		publish?: string[];
		put?: string | string[];
		root?: string;
		sub?: unknown;
		subscribe?: string[];
	}) => Promise<string>;
	verify: (key: {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		k: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "oct";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | PublicKey, token: string) => Promise<Omit<{
		exp?: number;
		get?: string | string[];
		iat?: number;
		nbf?: number;
		publish?: string[];
		put?: string | string[];
		root: string;
		subscribe?: string[];
	}, "put" | "get"> & {
		publish?: string[];
		subscribe?: string[];
	}>;
};

export type Key = z.infer<typeof KeySchema>;

export type KeyId = z.infer<typeof KeyIdSchema>;

export const KeyIdSchema: z.ZodMiniString<string>;

export const KeySchema: z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
	alg: z.ZodMiniEnum<{
		ES256: "ES256";
		ES384: "ES384";
		EdDSA: "EdDSA";
		HS256: "HS256";
		HS384: "HS384";
		HS512: "HS512";
		PS256: "PS256";
		PS384: "PS384";
		PS512: "PS512";
		RS256: "RS256";
		RS384: "RS384";
		RS512: "RS512";
	}>;
	k: z.ZodMiniString<string>;
	key_ops: z.ZodMiniDefault<z.ZodMiniArray<z.ZodMiniEnum<{
		decrypt: "decrypt";
		encrypt: "encrypt";
		sign: "sign";
		verify: "verify";
	}>>>;
	kid: z.ZodMiniOptional<z.ZodMiniString<string>>;
	kty: z.ZodMiniLiteral<"oct">;
	scope: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniObject<{
		get: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		publish: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		put: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		root: z.ZodMiniDefault<z.ZodMiniString<string>>;
		subscribe: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
	}, z.core.$strict>, z.ZodMiniTransform<Omit<{
		get?: ... | ...;
		publish?: ... | ...;
		put?: ... | ...;
		root: string;
		subscribe?: ... | ...;
	}, "get" | "put"> & {
		publish?: string[];
		subscribe?: string[];
	}, {
		get?: string[];
		publish?: string[];
		put?: string[];
		root: string;
		subscribe?: string[];
	}>>>;
}, z.core.$strip>, z.ZodMiniObject<{
	alg: z.ZodMiniEnum<{
		ES256: "ES256";
		ES384: "ES384";
		EdDSA: "EdDSA";
		HS256: "HS256";
		HS384: "HS384";
		HS512: "HS512";
		PS256: "PS256";
		PS384: "PS384";
		PS512: "PS512";
		RS256: "RS256";
		RS384: "RS384";
		RS512: "RS512";
	}>;
	d: z.ZodMiniOptional<z.ZodMiniString<string>>;
	dp: z.ZodMiniOptional<z.ZodMiniString<string>>;
	dq: z.ZodMiniOptional<z.ZodMiniString<string>>;
	e: z.ZodMiniString<string>;
	key_ops: z.ZodMiniDefault<z.ZodMiniArray<z.ZodMiniEnum<{
		decrypt: "decrypt";
		encrypt: "encrypt";
		sign: "sign";
		verify: "verify";
	}>>>;
	kid: z.ZodMiniOptional<z.ZodMiniString<string>>;
	kty: z.ZodMiniLiteral<"RSA">;
	n: z.ZodMiniString<string>;
	p: z.ZodMiniOptional<z.ZodMiniString<string>>;
	q: z.ZodMiniOptional<z.ZodMiniString<string>>;
	qi: z.ZodMiniOptional<z.ZodMiniString<string>>;
	scope: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniObject<{
		get: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		publish: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		put: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		root: z.ZodMiniDefault<z.ZodMiniString<string>>;
		subscribe: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
	}, z.core.$strict>, z.ZodMiniTransform<Omit<{
		get?: ... | ...;
		publish?: ... | ...;
		put?: ... | ...;
		root: string;
		subscribe?: ... | ...;
	}, "get" | "put"> & {
		publish?: string[];
		subscribe?: string[];
	}, {
		get?: string[];
		publish?: string[];
		put?: string[];
		root: string;
		subscribe?: string[];
	}>>>;
}, z.core.$strip>, z.ZodMiniObject<{
	alg: z.ZodMiniEnum<{
		ES256: "ES256";
		ES384: "ES384";
		EdDSA: "EdDSA";
		HS256: "HS256";
		HS384: "HS384";
		HS512: "HS512";
		PS256: "PS256";
		PS384: "PS384";
		PS512: "PS512";
		RS256: "RS256";
		RS384: "RS384";
		RS512: "RS512";
	}>;
	crv: z.ZodMiniEnum<{
		P-256: "P-256";
		P-384: "P-384";
	}>;
	d: z.ZodMiniOptional<z.ZodMiniString<string>>;
	key_ops: z.ZodMiniDefault<z.ZodMiniArray<z.ZodMiniEnum<{
		decrypt: "decrypt";
		encrypt: "encrypt";
		sign: "sign";
		verify: "verify";
	}>>>;
	kid: z.ZodMiniOptional<z.ZodMiniString<string>>;
	kty: z.ZodMiniLiteral<"EC">;
	scope: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniObject<{
		get: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		publish: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		put: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		root: z.ZodMiniDefault<z.ZodMiniString<string>>;
		subscribe: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
	}, z.core.$strict>, z.ZodMiniTransform<Omit<{
		get?: ... | ...;
		publish?: ... | ...;
		put?: ... | ...;
		root: string;
		subscribe?: ... | ...;
	}, "get" | "put"> & {
		publish?: string[];
		subscribe?: string[];
	}, {
		get?: string[];
		publish?: string[];
		put?: string[];
		root: string;
		subscribe?: string[];
	}>>>;
	x: z.ZodMiniString<string>;
	y: z.ZodMiniString<string>;
}, z.core.$strip>, z.ZodMiniObject<{
	alg: z.ZodMiniEnum<{
		ES256: "ES256";
		ES384: "ES384";
		EdDSA: "EdDSA";
		HS256: "HS256";
		HS384: "HS384";
		HS512: "HS512";
		PS256: "PS256";
		PS384: "PS384";
		PS512: "PS512";
		RS256: "RS256";
		RS384: "RS384";
		RS512: "RS512";
	}>;
	crv: z.ZodMiniLiteral<"Ed25519">;
	d: z.ZodMiniOptional<z.ZodMiniString<string>>;
	key_ops: z.ZodMiniDefault<z.ZodMiniArray<z.ZodMiniEnum<{
		decrypt: "decrypt";
		encrypt: "encrypt";
		sign: "sign";
		verify: "verify";
	}>>>;
	kid: z.ZodMiniOptional<z.ZodMiniString<string>>;
	kty: z.ZodMiniLiteral<"OKP">;
	scope: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniObject<{
		get: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		publish: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		put: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
		root: z.ZodMiniDefault<z.ZodMiniString<string>>;
		subscribe: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<...>>>;
	}, z.core.$strict>, z.ZodMiniTransform<Omit<{
		get?: ... | ...;
		publish?: ... | ...;
		put?: ... | ...;
		root: string;
		subscribe?: ... | ...;
	}, "get" | "put"> & {
		publish?: string[];
		subscribe?: string[];
	}, {
		get?: string[];
		publish?: string[];
		put?: string[];
		root: string;
		subscribe?: string[];
	}>>>;
	x: z.ZodMiniString<string>;
}, z.core.$strip>], "kty">;

export const KeySet: {
	find: (set: {
		keys: ({
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			k: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "oct";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			d?: string;
			dp?: string;
			dq?: string;
			e: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "RSA";
			n: string;
			p?: string;
			q?: string;
			qi?: string;
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "P-256" | "P-384";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "EC";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
			y: string;
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "Ed25519";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "OKP";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
		})[];
	}, kid: string) => {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		k: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "oct";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		d?: string;
		dp?: string;
		dq?: string;
		e: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "RSA";
		n: string;
		p?: string;
		q?: string;
		qi?: string;
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "P-256" | "P-384";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "EC";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
		y: string;
	} | {
		alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
		crv: "Ed25519";
		d?: string;
		key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
		kid?: string;
		kty: "OKP";
		scope?: Omit<{
			get?: string[];
			publish?: string[];
			put?: string[];
			root: string;
			subscribe?: string[];
		}, "put" | "get"> & {
			publish?: string[];
			subscribe?: string[];
		};
		x: string;
	} | undefined;
	parse: (jwks: string) => {
		keys: ({
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			k: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "oct";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			d?: string;
			dp?: string;
			dq?: string;
			e: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "RSA";
			n: string;
			p?: string;
			q?: string;
			qi?: string;
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "P-256" | "P-384";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "EC";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
			y: string;
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "Ed25519";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "OKP";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
		})[];
	};
	public: (set: {
		keys: ({
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			k: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "oct";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			d?: string;
			dp?: string;
			dq?: string;
			e: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "RSA";
			n: string;
			p?: string;
			q?: string;
			qi?: string;
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "P-256" | "P-384";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "EC";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
			y: string;
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "Ed25519";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "OKP";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
		})[];
	}) => PublicKeySet;
	sign: (set: {
		keys: ({
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			k: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "oct";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			d?: string;
			dp?: string;
			dq?: string;
			e: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "RSA";
			n: string;
			p?: string;
			q?: string;
			qi?: string;
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "P-256" | "P-384";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "EC";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
			y: string;
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "Ed25519";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "OKP";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
		})[];
	}, claims: {
		exp?: number;
		get?: string | string[];
		iat?: number;
		iss?: unknown;
		jti?: unknown;
		nbf?: number;
		publish?: string[];
		put?: string | string[];
		root?: string;
		sub?: unknown;
		subscribe?: string[];
	}) => Promise<string>;
	verify: (set: {
		keys: ({
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			k: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "oct";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			d?: string;
			dp?: string;
			dq?: string;
			e: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "RSA";
			n: string;
			p?: string;
			q?: string;
			qi?: string;
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "P-256" | "P-384";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "EC";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
			y: string;
		} | {
			alg: "ES256" | "ES384" | "EdDSA" | "HS256" | "HS384" | "HS512" | "PS256" | "PS384" | "PS512" | "RS256" | "RS384" | "RS512";
			crv: "Ed25519";
			d?: string;
			key_ops: ("decrypt" | "encrypt" | "sign" | "verify")[];
			kid?: string;
			kty: "OKP";
			scope?: Omit<{
				get?: ... | ...;
				publish?: ... | ...;
				put?: ... | ...;
				root: string;
				subscribe?: ... | ...;
			}, "put" | "get"> & {
				publish?: ...[];
				subscribe?: ...[];
			};
			x: string;
		})[];
	}, token: string) => Promise<Omit<{
		exp?: number;
		get?: string | string[];
		iat?: number;
		nbf?: number;
		publish?: string[];
		put?: string | string[];
		root: string;
		subscribe?: string[];
	}, "put" | "get"> & {
		publish?: string[];
		subscribe?: string[];
	}>;
};

export type KeySet = z.infer<typeof KeySetSchema>;

export const KeySetSchema: z.ZodMiniObject<{
	keys: z.ZodMiniArray<z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
		alg: z.ZodMiniEnum<{
			ES256: "ES256";
			ES384: "ES384";
			EdDSA: "EdDSA";
			HS256: "HS256";
			HS384: "HS384";
			HS512: "HS512";
			PS256: "PS256";
			PS384: "PS384";
			PS512: "PS512";
			RS256: "RS256";
			RS384: "RS384";
			RS512: "RS512";
		}>;
		k: z.ZodMiniString<string>;
		key_ops: z.ZodMiniDefault<z.ZodMiniArray<z.ZodMiniEnum<{
			decrypt: ...;
			encrypt: ...;
			sign: ...;
			verify: ...;
		}>>>;
		kid: z.ZodMiniOptional<z.ZodMiniString<string>>;
		kty: z.ZodMiniLiteral<"oct">;
		scope: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniObject<{
			get: ...;
			publish: ...;
			put: ...;
			root: ...;
			subscribe: ...;
		}, z.core.$strict>, z.ZodMiniTransform<... & ..., {
			get?: ...;
			publish?: ...;
			put?: ...;
			root: ...;
			subscribe?: ...;
		}>>>;
	}, z.core.$strip>, z.ZodMiniObject<{
		alg: z.ZodMiniEnum<{
			ES256: "ES256";
			ES384: "ES384";
			EdDSA: "EdDSA";
			HS256: "HS256";
			HS384: "HS384";
			HS512: "HS512";
			PS256: "PS256";
			PS384: "PS384";
			PS512: "PS512";
			RS256: "RS256";
			RS384: "RS384";
			RS512: "RS512";
		}>;
		d: z.ZodMiniOptional<z.ZodMiniString<string>>;
		dp: z.ZodMiniOptional<z.ZodMiniString<string>>;
		dq: z.ZodMiniOptional<z.ZodMiniString<string>>;
		e: z.ZodMiniString<string>;
		key_ops: z.ZodMiniDefault<z.ZodMiniArray<z.ZodMiniEnum<{
			decrypt: ...;
			encrypt: ...;
			sign: ...;
			verify: ...;
		}>>>;
		kid: z.ZodMiniOptional<z.ZodMiniString<string>>;
		kty: z.ZodMiniLiteral<"RSA">;
		n: z.ZodMiniString<string>;
		p: z.ZodMiniOptional<z.ZodMiniString<string>>;
		q: z.ZodMiniOptional<z.ZodMiniString<string>>;
		qi: z.ZodMiniOptional<z.ZodMiniString<string>>;
		scope: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniObject<{
			get: ...;
			publish: ...;
			put: ...;
			root: ...;
			subscribe: ...;
		}, z.core.$strict>, z.ZodMiniTransform<... & ..., {
			get?: ...;
			publish?: ...;
			put?: ...;
			root: ...;
			subscribe?: ...;
		}>>>;
	}, z.core.$strip>, z.ZodMiniObject<{
		alg: z.ZodMiniEnum<{
			ES256: "ES256";
			ES384: "ES384";
			EdDSA: "EdDSA";
			HS256: "HS256";
			HS384: "HS384";
			HS512: "HS512";
			PS256: "PS256";
			PS384: "PS384";
			PS512: "PS512";
			RS256: "RS256";
			RS384: "RS384";
			RS512: "RS512";
		}>;
		crv: z.ZodMiniEnum<{
			P-256: "P-256";
			P-384: "P-384";
		}>;
		d: z.ZodMiniOptional<z.ZodMiniString<string>>;
		key_ops: z.ZodMiniDefault<z.ZodMiniArray<z.ZodMiniEnum<{
			decrypt: ...;
			encrypt: ...;
			sign: ...;
			verify: ...;
		}>>>;
		kid: z.ZodMiniOptional<z.ZodMiniString<string>>;
		kty: z.ZodMiniLiteral<"EC">;
		scope: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniObject<{
			get: ...;
			publish: ...;
			put: ...;
			root: ...;
			subscribe: ...;
		}, z.core.$strict>, z.ZodMiniTransform<... & ..., {
			get?: ...;
			publish?: ...;
			put?: ...;
			root: ...;
			subscribe?: ...;
		}>>>;
		x: z.ZodMiniString<string>;
		y: z.ZodMiniString<string>;
	}, z.core.$strip>, z.ZodMiniObject<{
		alg: z.ZodMiniEnum<{
			ES256: "ES256";
			ES384: "ES384";
			EdDSA: "EdDSA";
			HS256: "HS256";
			HS384: "HS384";
			HS512: "HS512";
			PS256: "PS256";
			PS384: "PS384";
			PS512: "PS512";
			RS256: "RS256";
			RS384: "RS384";
			RS512: "RS512";
		}>;
		crv: z.ZodMiniLiteral<"Ed25519">;
		d: z.ZodMiniOptional<z.ZodMiniString<string>>;
		key_ops: z.ZodMiniDefault<z.ZodMiniArray<z.ZodMiniEnum<{
			decrypt: ...;
			encrypt: ...;
			sign: ...;
			verify: ...;
		}>>>;
		kid: z.ZodMiniOptional<z.ZodMiniString<string>>;
		kty: z.ZodMiniLiteral<"OKP">;
		scope: z.ZodMiniOptional<z.ZodMiniPipe<z.ZodMiniObject<{
			get: ...;
			publish: ...;
			put: ...;
			root: ...;
			subscribe: ...;
		}, z.core.$strict>, z.ZodMiniTransform<... & ..., {
			get?: ...;
			publish?: ...;
			put?: ...;
			root: ...;
			subscribe?: ...;
		}>>>;
		x: z.ZodMiniString<string>;
	}, z.core.$strip>], "kty">>;
}, z.core.$strip>;

export type Operation = z.infer<typeof OperationSchema>;

export const OperationSchema: z.ZodMiniEnum<{
	decrypt: "decrypt";
	encrypt: "encrypt";
	sign: "sign";
	verify: "verify";
}>;

export { Pattern } from "@moq/pattern";

export const PatternListSchema: z.ZodMiniArray<z.ZodMiniString<string>>;

export { Patterns } from "@moq/pattern";

export type Peer = z.infer<typeof PeerSchema>;

export const PeerSchema: z.ZodMiniObject<{
	expires: z.ZodMiniOptional<z.ZodMiniInt>;
	fingerprint: z.ZodMiniString<string>;
	issuer: z.ZodMiniString<string>;
	name: z.ZodMiniString<string>;
}, z.core.$strip>;

export interface Permissions {
	publish: Patterns;
	subscribe: Patterns;
}

export type PublicKey = Omit<AsymmetricKey, "d" | "p" | "q" | "dp" | "dq" | "qi">;

export interface PublicKeySet {
	keys: PublicKey[];
}

export type Refusal = z.infer<typeof RefusalSchema>;

export const RefusalSchema: z.ZodMiniObject<{
	reason: z.ZodMiniPipe<z.ZodMiniString<string>, z.ZodMiniCatch<z.ZodMiniEnum<{
		expired: "expired";
		forbidden: "forbidden";
		malformed: "malformed";
		refused: "refused";
	}>>>;
	root: z.ZodMiniOptional<z.ZodMiniString<string>>;
	tier: z.ZodMiniOptional<z.ZodMiniString<string>>;
}, z.core.$strip>;

export type Request = z.infer<typeof RequestSchema>;

export const RequestSchema: z.ZodMiniDiscriminatedUnion<[z.ZodMiniObject<{
	alpn: z.ZodMiniOptional<z.ZodMiniString<string>>;
	event: z.ZodMiniLiteral<"connect">;
	id: z.ZodMiniString<string>;
	implementation: z.ZodMiniOptional<z.ZodMiniString<string>>;
	local: z.ZodMiniOptional<z.ZodMiniString<string>>;
	node: z.ZodMiniString<string>;
	path: z.ZodMiniString<string>;
	query: z.ZodMiniOptional<z.ZodMiniString<string>>;
	remote: z.ZodMiniOptional<z.ZodMiniString<string>>;
	role: z.ZodMiniOptional<z.ZodMiniEnum<{
		publisher: "publisher";
		subscriber: "subscriber";
	}>>;
	server_name: z.ZodMiniOptional<z.ZodMiniString<string>>;
	tls: z.ZodMiniOptional<z.ZodMiniObject<{
		expires: z.ZodMiniOptional<z.ZodMiniInt>;
		fingerprint: z.ZodMiniString<string>;
		issuer: z.ZodMiniString<string>;
		name: z.ZodMiniString<string>;
	}, z.core.$strip>>;
	token: z.ZodMiniOptional<z.ZodMiniObject<{
		kind: z.ZodMiniInt;
		value: z.ZodMiniString<string>;
	}, z.core.$strip>>;
	transport: z.ZodMiniPipe<z.ZodMiniString<string>, z.ZodMiniCatch<z.ZodMiniEnum<{
		http: "http";
		iroh: "iroh";
		quic: "quic";
		rtmp: "rtmp";
		srt: "srt";
		tcp: "tcp";
		unix: "unix";
		unknown: "unknown";
		webrtc: "webrtc";
		websocket: "websocket";
		webtransport: "webtransport";
	}>>>;
}, z.core.$strip>, z.ZodMiniObject<{
	alpn: z.ZodMiniOptional<z.ZodMiniString<string>>;
	event: z.ZodMiniLiteral<"revalidate">;
	id: z.ZodMiniString<string>;
	implementation: z.ZodMiniOptional<z.ZodMiniString<string>>;
	local: z.ZodMiniOptional<z.ZodMiniString<string>>;
	node: z.ZodMiniString<string>;
	path: z.ZodMiniString<string>;
	query: z.ZodMiniOptional<z.ZodMiniString<string>>;
	remote: z.ZodMiniOptional<z.ZodMiniString<string>>;
	role: z.ZodMiniOptional<z.ZodMiniEnum<{
		publisher: "publisher";
		subscriber: "subscriber";
	}>>;
	server_name: z.ZodMiniOptional<z.ZodMiniString<string>>;
	tls: z.ZodMiniOptional<z.ZodMiniObject<{
		expires: z.ZodMiniOptional<z.ZodMiniInt>;
		fingerprint: z.ZodMiniString<string>;
		issuer: z.ZodMiniString<string>;
		name: z.ZodMiniString<string>;
	}, z.core.$strip>>;
	token: z.ZodMiniOptional<z.ZodMiniObject<{
		kind: z.ZodMiniInt;
		value: z.ZodMiniString<string>;
	}, z.core.$strip>>;
	transport: z.ZodMiniPipe<z.ZodMiniString<string>, z.ZodMiniCatch<z.ZodMiniEnum<{
		http: "http";
		iroh: "iroh";
		quic: "quic";
		rtmp: "rtmp";
		srt: "srt";
		tcp: "tcp";
		unix: "unix";
		unknown: "unknown";
		webrtc: "webrtc";
		websocket: "websocket";
		webtransport: "webtransport";
	}>>>;
}, z.core.$strip>, z.ZodMiniObject<{
	alpn: z.ZodMiniOptional<z.ZodMiniString<string>>;
	bytes: z.ZodMiniObject<{
		received: z.ZodMiniInt;
		sent: z.ZodMiniInt;
	}, z.core.$strip>;
	duration: z.ZodMiniNumber<number>;
	event: z.ZodMiniLiteral<"end">;
	id: z.ZodMiniString<string>;
	implementation: z.ZodMiniOptional<z.ZodMiniString<string>>;
	local: z.ZodMiniOptional<z.ZodMiniString<string>>;
	node: z.ZodMiniString<string>;
	path: z.ZodMiniString<string>;
	query: z.ZodMiniOptional<z.ZodMiniString<string>>;
	reason: z.ZodMiniString<string>;
	remote: z.ZodMiniOptional<z.ZodMiniString<string>>;
	role: z.ZodMiniOptional<z.ZodMiniEnum<{
		publisher: "publisher";
		subscriber: "subscriber";
	}>>;
	server_name: z.ZodMiniOptional<z.ZodMiniString<string>>;
	tls: z.ZodMiniOptional<z.ZodMiniObject<{
		expires: z.ZodMiniOptional<z.ZodMiniInt>;
		fingerprint: z.ZodMiniString<string>;
		issuer: z.ZodMiniString<string>;
		name: z.ZodMiniString<string>;
	}, z.core.$strip>>;
	token: z.ZodMiniOptional<z.ZodMiniObject<{
		kind: z.ZodMiniInt;
		value: z.ZodMiniString<string>;
	}, z.core.$strip>>;
	transport: z.ZodMiniPipe<z.ZodMiniString<string>, z.ZodMiniCatch<z.ZodMiniEnum<{
		http: "http";
		iroh: "iroh";
		quic: "quic";
		rtmp: "rtmp";
		srt: "srt";
		tcp: "tcp";
		unix: "unix";
		unknown: "unknown";
		webrtc: "webrtc";
		websocket: "websocket";
		webtransport: "webtransport";
	}>>>;
}, z.core.$strip>], "event">;

export type Role = z.infer<typeof RoleSchema>;

export const RoleSchema: z.ZodMiniEnum<{
	publisher: "publisher";
	subscriber: "subscriber";
}>;

export type Scope = z.output<typeof ScopeSchema>;

export const ScopeSchema: z.ZodMiniPipe<z.ZodMiniObject<{
	get: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<string>>>;
	publish: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<string>>>;
	put: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<string>>>;
	root: z.ZodMiniDefault<z.ZodMiniString<string>>;
	subscribe: z.ZodMiniOptional<z.ZodMiniArray<z.ZodMiniString<string>>>;
}, z.core.$strict>, z.ZodMiniTransform<Omit<{
	get?: string[];
	publish?: string[];
	put?: string[];
	root: string;
	subscribe?: string[];
}, "get" | "put"> & {
	publish?: string[];
	subscribe?: string[];
}, {
	get?: string[];
	publish?: string[];
	put?: string[];
	root: string;
	subscribe?: string[];
}>>;

export { Segment } from "@moq/pattern";

export { Specificity } from "@moq/pattern";

export type SymmetricKey = Extract<Key, {
	kty: "oct";
}>;

export type Token = z.infer<typeof TokenSchema>;

export const TokenSchema: z.ZodMiniObject<{
	kind: z.ZodMiniInt;
	value: z.ZodMiniString<string>;
}, z.core.$strip>;

export type Transport = z.infer<typeof TransportSchema>;

export const TransportSchema: z.ZodMiniPipe<z.ZodMiniString<string>, z.ZodMiniCatch<z.ZodMiniEnum<{
	http: "http";
	iroh: "iroh";
	quic: "quic";
	rtmp: "rtmp";
	srt: "srt";
	tcp: "tcp";
	unix: "unix";
	unknown: "unknown";
	webrtc: "webrtc";
	websocket: "websocket";
	webtransport: "webtransport";
}>>>;

export function authorize(claims: Omit<{
	exp?: number;
	get?: string | string[];
	iat?: number;
	nbf?: number;
	publish?: string[];
	put?: string | string[];
	root: string;
	subscribe?: string[];
}, "put" | "get"> & {
	publish?: string[];
	subscribe?: string[];
}, path: string): Permissions;

export { compareSpecificity } from "@moq/pattern";

export function patterns(texts: readonly string[] | undefined): Patterns;

export function scopeAllows(scope: Omit<{
	get?: string[];
	publish?: string[];
	put?: string[];
	root: string;
	subscribe?: string[];
}, "put" | "get"> & {
	publish?: string[];
	subscribe?: string[];
}, claims: Omit<{
	exp?: number;
	get?: string | string[];
	iat?: number;
	nbf?: number;
	publish?: string[];
	put?: string | string[];
	root: string;
	subscribe?: string[];
}, "put" | "get"> & {
	publish?: string[];
	subscribe?: string[];
}): boolean;
```
