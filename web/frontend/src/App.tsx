import { Component, lazy } from 'solid-js';

import { Route, Routes } from 'solid-app-router';


const UserPage = lazy(() => import("./pages/user"));


const App: Component = () => {
  return (
    <>
      <Routes base='act/'>
        <Route path="user/:name" component={UserPage} />
      </Routes>
    </>
  );
};

export default App;
