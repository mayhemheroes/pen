Feature: Union
  Background:
    Given a file named "pen.json" with:
    """json
    {
      "dependencies": {
        "System": "file://pen-root/lib/os"
      }
    }
    """

  Scenario: Downcast a union to a list
    Given a file named "Main.pen" with:
    """pen
    import System'Os

    main = \(os Os'Os) number {
      x = if true {
        [none;]
      } else {
        none
      }

      if x = x; [none] {
        none
      } else {
        none
      }

      0
    }
    """
    When I successfully run `pen build`
    Then I successfully run `./app`