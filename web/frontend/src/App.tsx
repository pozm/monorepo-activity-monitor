import { Component, lazy } from 'solid-js';

import { Route, Routes } from 'solid-app-router';


const UserPage = lazy(() => import("./pages/user"));
const UsersPage = lazy(() => import("./pages/users"));


const App: Component = () => {
  return (
    <>
      <Routes base='act/'>
        <Route path="user/:name" component={UserPage} />
        <Route path="user/" component={UsersPage} />
        <Route path="users/" component={UsersPage} />
      </Routes>
    </>
  );
};

export default App;
