# Full Site Migration Tracker

## Goal

Close migration planning for all workflow-packaged sites with one execution tracker that assigns every site a wave, target runtime direction, ownership expectation, and smoke entry point.

Status legend:

- `implemented`: already has a meaningful migrated reference path

Repo strategy legend:

- `builtin`: keep in the builtin workflow catalog
- `builtin+conditional-git`: builtin first, external git repo only when ownership or release cadence justifies it
- `builtin-native-ref`: keep builtin and use as native reference site

## Wave 1: Reference Baselines

| Site | Status | Commands | Current mix | Target direction | Repo strategy | Suggested smoke |
| --- | --- | ---: | --- | --- | --- | --- |
| `wikipedia` | `implemented` | 4 | `script x4` | public `workflow-script` baseline | `builtin` | `wikipedia/summary` |
| `hackernews` | `implemented` | 8 | `script x8` | public package-local `workflow-script` regression anchor | `builtin` | `hackernews/top` |
| `bilibili` | `implemented` | 11 | `script x10`, `native x1` | keep native reference, selectively add `script` helpers | `builtin-native-ref` | `bilibili/feed` |

## Wave 2: Public Structured Sites

| Site | Status | Commands | Current mix | Target direction | Repo strategy | Suggested smoke |
| --- | --- | ---: | --- | --- | --- | --- |
| `apple-podcasts` | `implemented` | 3 | `script x3` | move search/list flows to `workflow-script` only if helper reuse appears | `builtin` | `apple-podcasts/top` |
| `arxiv` | `implemented` | 2 | `script x2` | small public `workflow-script` candidate | `builtin` | `arxiv/search` |
| `bbc` | `implemented` | 1 | `script x1` | keep simple package-local `workflow-script` unless public helper reuse emerges | `builtin` | `bbc/news` |
| `devto` | `implemented` | 5 | `script x5` | public feed/search `workflow-script` candidate | `builtin` | `devto/feed` |
| `google` | `implemented` | 4 | `script x4` | keep builtin, convert search/news helpers only if normalization value is clear | `builtin` | `google/search` |
| `hf` | `implemented` | 1 | `script x1` | keep simple package-local `workflow-script` | `builtin` | `hf/top` |
| `linux-do` | `implemented` | 6 | `script x6` | public forum helper reuse via `workflow-script` if needed | `builtin` | `linux-do/hot` |
| `lobsters` | `implemented` | 5 | `script x5` | public list/search `workflow-script` candidate | `builtin` | `lobsters/hot` |
| `reuters` | `implemented` | 2 | `script x2` | keep builtin, low-risk public script candidate later | `builtin` | `reuters/news` |
| `stackoverflow` | `implemented` | 5 | `script x5` | public structured `workflow-script` candidate | `builtin` | `stackoverflow/search` |
| `steam` | `implemented` | 1 | `script x1` | keep simple package-local `workflow-script` | `builtin` | `steam/top-sellers` |
| `xiaoyuzhou` | `implemented` | 3 | `script x3` | public content extraction `workflow-script` candidate | `builtin` | `xiaoyuzhou/podcast` |

## Wave 3: Content And Market Sites

| Site | Status | Commands | Current mix | Target direction | Repo strategy | Suggested smoke |
| --- | --- | ---: | --- | --- | --- | --- |
| `barchart` | `implemented` | 4 | `script x4` | shared quote/options helpers via `workflow-script` | `builtin` | `barchart/options` |
| `bloomberg` | `implemented` | 10 | `script x10` | content/feed normalization helpers via `workflow-script` | `builtin` | `bloomberg/markets` |
| `coupang` | `implemented` | 1 | `script x1` | keep simple builtin `workflow-script` unless more commands land | `builtin` | `coupang/search` |
| `douban` | `implemented` | 7 | `script x7` | search/hot list helpers via `workflow-script` | `builtin` | `douban/movie-hot` |
| `medium` | `implemented` | 3 | `script x3` | content/feed helper reuse via `workflow-script` | `builtin` | `medium/feed` |
| `sinablog` | `implemented` | 4 | `script x4` | article/search normalization via `workflow-script` | `builtin` | `sinablog/hot` |
| `sinafinance` | `implemented` | 1 | `script x1` | keep simple builtin `workflow-script` | `builtin` | `sinafinance/news` |
| `smzdm` | `implemented` | 1 | `script x1` | keep simple builtin `workflow-script` | `builtin` | `smzdm/search` |
| `substack` | `implemented` | 3 | `script x3` | publication/feed helper reuse via `workflow-script` | `builtin` | `substack/feed` |
| `weibo` | `implemented` | 2 | `script x2` | compact `workflow-script` candidate if auth/session reuse is needed | `builtin` | `weibo/hot` |
| `weread` | `implemented` | 7 | `script x7` | notebook/highlight/book helpers via `workflow-script` | `builtin` | `weread/search` |
| `xueqiu` | `implemented` | 7 | `script x7` | market/session helpers via `workflow-script` | `builtin+conditional-git` | `xueqiu/feed` |
| `yahoo-finance` | `implemented` | 1 | `script x1` | keep simple builtin `workflow-script` unless quote helper reuse grows | `builtin` | `yahoo-finance/quote` |
| `youtube` | `implemented` | 3 | `script x3` | search/video/transcript helper reuse via `workflow-script` | `builtin` | `youtube/search` |
| `zhihu` | `implemented` | 3 | `script x3` | search/question/hot helper reuse via `workflow-script` | `builtin` | `zhihu/hot` |

## Wave 4: Auth And Workspace Helpers

| Site | Status | Commands | Current mix | Target direction | Repo strategy | Suggested smoke |
| --- | --- | ---: | --- | --- | --- | --- |
| `discord-app` | `implemented` | 7 | `script x7` | channel/member/read helpers via `workflow-script` | `builtin` | `discord-app/channels` |
| `doubao` | `implemented` | 5 | `script x5` | ask/read/session helpers via `workflow-script` | `builtin` | `doubao/ask` |
| `grok` | `implemented` | 1 | `script x1` | keep builtin, add helper wrapper only if more commands arrive | `builtin` | `grok/ask` |
| `jike` | `implemented` | 10 | `script x10` | feed/comment/profile helpers via `workflow-script` | `builtin` | `jike/feed` |
| `jimeng` | `implemented` | 2 | `script x2` | generation/history helper reuse via `workflow-script` | `builtin` | `jimeng/history` |
| `notion` | `implemented` | 7 | `script x7` | UI/session/page helpers via `workflow-script` | `builtin` | `notion/search` |
| `reddit` | `implemented` | 15 | `script x15` | read/search/user helpers via `workflow-script` | `builtin` | `reddit/frontpage` |
| `v2ex` | `implemented` | 11 | `script x11` | topic/member/feed helpers via `workflow-script` | `builtin` | `v2ex/hot` |

## Wave 5: High-Change Social And Override Candidates

| Site | Status | Commands | Current mix | Target direction | Repo strategy | Suggested smoke |
| --- | --- | ---: | --- | --- | --- | --- |
| `boss` | `implemented` | 15 | `script x15` | list/detail/chat helpers via `workflow-script` | `builtin+conditional-git` | `boss/joblist` |
| `facebook` | `implemented` | 10 | `script x10` | keep builtin first, expect helper-heavy `workflow-script` migration | `builtin+conditional-git` | `facebook/feed` |
| `instagram` | `implemented` | 14 | `script x14` | helper-heavy `workflow-script`, external repo only if selector churn demands it | `builtin+conditional-git` | `instagram/explore` |
| `tiktok` | `implemented` | 15 | `script x15` | helper-heavy `workflow-script`, likely high-change site | `builtin+conditional-git` | `tiktok/explore` |
| `twitter` | `implemented` | 22 | `script x22` | timeline/profile/write helpers via `workflow-script` first | `builtin+conditional-git` | `twitter/timeline` |
| `xiaohongshu` | `implemented` | 10 | `script x10` | creator/note helpers via `workflow-script` | `builtin+conditional-git` | `xiaohongshu/search` |

## Global Decisions

- every site now has an assigned migration wave
- builtin workflow packages remain the default ownership model for all sites
- external git repos are reserved for high-change sites with clear ownership and release cadence pressure
- `bilibili` remains the native reference site
- `wikipedia` remains the public script reference site
- `hackernews` remains the public low-risk script regression anchor
- all workflow commands now resolve package-local assets under `workflows/<site>/scripts` or native handlers
- current runtime shape is `261 workflow-script + 1 workflow-native`

## Completion Criteria For Planning

Planning is considered complete when:

- every workflow site has a wave
- every workflow site has a target runtime direction
- every workflow site has a default repo strategy
- every workflow site has a suggested smoke command

This tracker now satisfies that bar for the current 44-site catalog.
