# `@moq/msf`

```ts
export type Catalog = z.infer<typeof CatalogSchema>;

export const CatalogSchema: z.ZodMiniObject<{
	ext: z.ZodMiniOptional<z.ZodMiniRecord<z.ZodMiniString<string>, z.ZodMiniUnknown>>;
	tracks: z.ZodMiniArray<z.ZodMiniObject<{
		altGroup: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		bitrate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		channelConfig: z.ZodMiniOptional<z.ZodMiniString<string>>;
		codec: z.ZodMiniOptional<z.ZodMiniString<string>>;
		delay: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		height: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		initData: z.ZodMiniOptional<z.ZodMiniString<string>>;
		isLive: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
		jitter: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		label: z.ZodMiniOptional<z.ZodMiniString<string>>;
		lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
		maxGrpSapStartingType: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		maxObjSapStartingType: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		name: z.ZodMiniString<string>;
		namespace: z.ZodMiniOptional<z.ZodMiniString<string>>;
		packaging: z.ZodMiniUnion<readonly [z.ZodMiniEnum<{
			cmaf: "cmaf";
			eventtimeline: "eventtimeline";
			legacy: "legacy";
			loc: "loc";
			mediatimeline: "mediatimeline";
		}>, z.ZodMiniString<string>]>;
		renderGroup: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		role: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniEnum<{
			audio: ...;
			audiodescription: ...;
			caption: ...;
			signlanguage: ...;
			subtitle: ...;
			video: ...;
		}>, z.ZodMiniString<string>]>>;
		samplerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
		width: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	}, z.core.$strip>>;
}, z.core.$strip>;

export type Packaging = z.infer<typeof PackagingSchema>;

export const PackagingSchema: z.ZodMiniUnion<readonly [z.ZodMiniEnum<{
	cmaf: "cmaf";
	eventtimeline: "eventtimeline";
	legacy: "legacy";
	loc: "loc";
	mediatimeline: "mediatimeline";
}>, z.ZodMiniString<string>]>;

export type Role = z.infer<typeof RoleSchema>;

export const RoleSchema: z.ZodMiniUnion<readonly [z.ZodMiniEnum<{
	audio: "audio";
	audiodescription: "audiodescription";
	caption: "caption";
	signlanguage: "signlanguage";
	subtitle: "subtitle";
	video: "video";
}>, z.ZodMiniString<string>]>;

export type Track = z.infer<typeof TrackSchema>;

export const TrackSchema: z.ZodMiniObject<{
	altGroup: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	bitrate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	channelConfig: z.ZodMiniOptional<z.ZodMiniString<string>>;
	codec: z.ZodMiniOptional<z.ZodMiniString<string>>;
	delay: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	framerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	height: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	initData: z.ZodMiniOptional<z.ZodMiniString<string>>;
	isLive: z.ZodMiniOptional<z.ZodMiniBoolean<boolean>>;
	jitter: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	label: z.ZodMiniOptional<z.ZodMiniString<string>>;
	lang: z.ZodMiniOptional<z.ZodMiniString<string>>;
	maxGrpSapStartingType: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	maxObjSapStartingType: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	name: z.ZodMiniString<string>;
	namespace: z.ZodMiniOptional<z.ZodMiniString<string>>;
	packaging: z.ZodMiniUnion<readonly [z.ZodMiniEnum<{
		cmaf: "cmaf";
		eventtimeline: "eventtimeline";
		legacy: "legacy";
		loc: "loc";
		mediatimeline: "mediatimeline";
	}>, z.ZodMiniString<string>]>;
	renderGroup: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	role: z.ZodMiniOptional<z.ZodMiniUnion<readonly [z.ZodMiniEnum<{
		audio: "audio";
		audiodescription: "audiodescription";
		caption: "caption";
		signlanguage: "signlanguage";
		subtitle: "subtitle";
		video: "video";
	}>, z.ZodMiniString<string>]>>;
	samplerate: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
	width: z.ZodMiniOptional<z.ZodMiniNumber<number>>;
}, z.core.$strip>;

export const VERSION: "draft-01";

export function decode(raw: Uint8Array): {
	ext?: Record<string, unknown>;
	tracks: {
		altGroup?: number;
		bitrate?: number;
		channelConfig?: string;
		codec?: string;
		delay?: number;
		framerate?: number;
		height?: number;
		initData?: string;
		isLive?: boolean;
		jitter?: number;
		label?: string;
		lang?: string;
		maxGrpSapStartingType?: number;
		maxObjSapStartingType?: number;
		name: string;
		namespace?: string;
		packaging: string;
		renderGroup?: number;
		role?: string;
		samplerate?: number;
		width?: number;
	}[];
};

export function encode(catalog: {
	ext?: Record<string, unknown>;
	tracks: {
		altGroup?: number;
		bitrate?: number;
		channelConfig?: string;
		codec?: string;
		delay?: number;
		framerate?: number;
		height?: number;
		initData?: string;
		isLive?: boolean;
		jitter?: number;
		label?: string;
		lang?: string;
		maxGrpSapStartingType?: number;
		maxObjSapStartingType?: number;
		name: string;
		namespace?: string;
		packaging: string;
		renderGroup?: number;
		role?: string;
		samplerate?: number;
		width?: number;
	}[];
}): Uint8Array;

export function fetch(track: Ordered): Promise<{
	ext?: Record<string, unknown>;
	tracks: {
		altGroup?: number;
		bitrate?: number;
		channelConfig?: string;
		codec?: string;
		delay?: number;
		framerate?: number;
		height?: number;
		initData?: string;
		isLive?: boolean;
		jitter?: number;
		label?: string;
		lang?: string;
		maxGrpSapStartingType?: number;
		maxObjSapStartingType?: number;
		name: string;
		namespace?: string;
		packaging: string;
		renderGroup?: number;
		role?: string;
		samplerate?: number;
		width?: number;
	}[];
} | undefined>;
```
