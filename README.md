struct mock

mock対象のstructの関数で
- mock対象の型以外を返す場合
  - 実際のオブジェクトを使用しておけば良い
  - もし、他のテストでモックになるとしたらuseの位置をずらせばよい

- mock対象の型を返す場合
  - 実際のオブジェクトは使えない
    - すべてmock型になる
  - assert_eq!()できない
    - partialEq, Eqが実装されていない
      - 作者としては、何をEqするべきか不明なので実装していない
        - 実装するなら自分でやって欲しいらしい
          - めんどくさい
  - Rcの参照先を比較すると言う手もあるかもしれない
    - 型があうのか？

  - 型情報のみのassertがあるか？
  https://github.com/Metaswitch/assert-type-eq-rs

  https://github.com/paholg/typenum