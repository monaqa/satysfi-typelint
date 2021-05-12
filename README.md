# satysfi-typelint

SATySFi の Type Error だけを抽出し、 linter の形式に落とし込むツール。手抜き。

[SATySFi Language Server](https://github.com/monaqa/satysfi-language-server)
に型検査器を入れるまでのあいだ型チェックを diagnostics 表示できないのは不便なので、
型チェックだけは
[efm-langserver](https://github.com/mattn/efm-langserver)
に任せることにした。

## 必要なもの

- [SATySFi](https://github.com/gfngfn/SATySFi) 処理系（`satysfi` コマンドにパスを通して executable にしておく）
- [efm-langserver](https://github.com/mattn/efm-langserver)

## 設定

`$XDG_CONFIG_HOME/efm-langserver/config.yaml` に以下の設定を入れる。

```yaml
version: 2

tools:
  satysfi: &satysfi
    # lint-command: "satysfi_lint"
    lint-command: "satysfi-typelint"

languages:
  satysfi:
    - <<: *satysfi
```

## 注意

もうどうしようもないくらい手抜き。
