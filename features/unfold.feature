Feature: unfold folder list

  Rule: replaces the current folder list with all subfolders that match the given condition

    @this
    Scenario:
      Given I am in the "monorepo" example folder
      And no mrt configuration
      When running "m unfold ls Makefile"
      Then it prints:
        """
        .....

        Limiting execution to 4/2 folders:
        1. /home/kevlar/mrt/examples/monorepo/product1
        2. /home/kevlar/mrt/examples/monorepo/product1/backend
        3. /home/kevlar/mrt/examples/monorepo/product1/frontend
        4. /home/kevlar/mrt/examples/monorepo/product2
        """
      And it returns "success"
