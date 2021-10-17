---
title: Botの招待
description: '002'
---

このサービスの主要な機能であるカレンダー機能を使うにはまず，DiscordのサーバーにBotを追加する必要があります．

<Btn
  outlined
  color="secondary"
  :external="true"
  to="https://discord.com/api/oauth2/authorize?client_id=771795045543313409&permissions=537193536&scope=bot">
  こちらから招待できます
</Btn>

また，ログイン後でしたら，サーバー一覧のページからまだBotを導入していないサーバーのうちあなたが管理権限を持っているサーバーが灰色で表示されます．

<VImage src="/docs/serverselect.png"></VImage>

そちらをクリックすることでもBotの招待が行えます．

招待画面ではこのBotを招待するサーバーを選択したのち，このBotに付与する権限についての確認画面へと遷移します．

<VImage src="/docs/permissions.png"></VImage>

最低限本Botの機能を利用する際には「メッセージを読む」「メッセージの送信」「埋め込みリンク」の権限が必要です．他の権限はOFFにしても構いませんが，その際は一部機能の利用が出来なくなります．ご了承ください．

最後にその次の画面で「Botではないこと」を証明するチェックボックスにチェックを入れれば導入完了です！

導入が終わった後のブラウザのタブは閉じてしまって構いません．
