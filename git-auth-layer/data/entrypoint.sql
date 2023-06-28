INSERT INTO public.users
(id, username, email, password, is_admin)
VALUES(1, 'userA', 'userA@user.fr', 'secret', false);

INSERT INTO public.users
(id, username, email, password, is_admin)
VALUES(2, 'userB', 'userB@user.fr', 'secret', false);

INSERT INTO public.projects
(id, uuid, name, user_id)
VALUES(1, '210e364a-3a07-43ba-85b8-2e1c646bd39a', 'repoA', 1);

INSERT INTO public.projects
(id, uuid, name, user_id)
VALUES(2, '4773ddcb-a600-49be-ad63-b769a1ad1eec', 'repoB', 2);
