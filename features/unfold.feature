Feature: unfold folder list

  Rule: replaces the current folder list with all subfolders that match the given condition

    Scenario: no previous limit
      Given I am in the "monorepo" example folder
      And no mrt configuration
      When running "m unfold ls Makefile"
      Then it prints:
        """
        .....

        Unfolding execution to 5 subfolders:
        1. {{examples_dir}}
        2. {{examples_dir}}/product1
        3. {{examples_dir}}/product1/backend
        4. {{examples_dir}}/product1/frontend
        5. {{examples_dir}}/product2
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1/10: cd {{examples_dir}}

        step 2/10: run pwd
        {{examples_dir}}

        step 3/10: cd {{examples_dir}}/product1

        step 4/10: run pwd
        {{examples_dir}}/product1

        step 5/10: cd {{examples_dir}}/product1/backend

        step 6/10: run pwd
        {{examples_dir}}/product1/backend

        step 7/10: cd {{examples_dir}}/product1/frontend

        step 8/10: run pwd
        {{examples_dir}}/product1/frontend

        step 9/10: cd {{examples_dir}}/product2

        step 10/10: run pwd
        {{examples_dir}}/product2

        ALL DONE
        """

    Scenario: has previous limit
      Given I am in the "monorepo" example folder
      And no mrt configuration
      When running "m only test -d frontend"
      And running "m unfold ls Makefile"
      Then it prints:
        """
        ....

        Unfolding the existing limit of 1/2 top-level folders to 3 subfolders:
        1. {{examples_dir}}/product1
        2. {{examples_dir}}/product1/backend
        3. {{examples_dir}}/product1/frontend
        """
      And it returns "success"
      When running "m run pwd"
      Then it prints:
        """
        step 1/6: cd {{examples_dir}}/product1

        step 2/6: run pwd
        {{examples_dir}}/product1

        step 3/6: cd {{examples_dir}}/product1/backend

        step 4/6: run pwd
        {{examples_dir}}/product1/backend

        step 5/6: cd {{examples_dir}}/product1/frontend

        step 6/6: run pwd
        {{examples_dir}}/product1/frontend

        ALL DONE
        """
