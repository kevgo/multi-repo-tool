Feature: unfold folder list

  Rule: replaces the current folder list with all subfolders that match the given condition

    Scenario: no previous limit
      Given I am in the "monorepo" example folder
      And no mrt configuration
      When running "m unfold ls Makefile"
      Then it prints:
        """
        .....

        Unfolding execution to 5 folders:
        1. /home/kevlar/mrt/examples/monorepo
        2. /home/kevlar/mrt/examples/monorepo/product1
        3. /home/kevlar/mrt/examples/monorepo/product1/backend
        4. /home/kevlar/mrt/examples/monorepo/product1/frontend
        5. /home/kevlar/mrt/examples/monorepo/product2
        """
      And it returns "success"

    Scenario: has previous limit
      Given I am in the "monorepo" example folder
      And no mrt configuration
      When running "m only test -d frontend"
      And running "m unfold ls Makefile"
      Then it prints:
        """
        ....

        Unfolding the existing limit of 1/2 folders to 3 folders:
        1. /home/kevlar/mrt/examples/monorepo/product1
        2. /home/kevlar/mrt/examples/monorepo/product1/backend
        3. /home/kevlar/mrt/examples/monorepo/product1/frontend
        """
      And it returns "success"
